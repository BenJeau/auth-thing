use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{
        authorization::{Basic, Bearer, Credentials},
        Authorization, UserAgent,
    },
    TypedHeader,
};
use database::{logic, models};
use http::HeaderValue;
use password::verify_password;
use std::net::SocketAddr;
use tracing::Span;

use crate::{Error, Result, ServerState};

#[derive(Clone, PartialEq, Debug)]
pub struct Token(String);

impl Credentials for Token {
    const SCHEME: &'static str = "Token";

    fn decode(value: &HeaderValue) -> Option<Self> {
        let bytes = &value.as_bytes()["Token ".len()..];
        let decoded = String::from_utf8(bytes.to_vec()).ok()?;
        Some(Self(decoded))
    }

    fn encode(&self) -> HeaderValue {
        HeaderValue::from_str(&format!("Token {}", self.0)).unwrap()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Realm(i64);

const REALM: &str = "x-auth-realm";
static REALM_HEADER_NAME: headers::HeaderName = headers::HeaderName::from_static(REALM);

impl Realm {
    pub fn into_inner(self) -> i64 {
        self.0
    }
}

impl headers::Header for Realm {
    fn name() -> &'static headers::HeaderName {
        &REALM_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> std::result::Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        let realm = value
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .parse()
            .map_err(|_| headers::Error::invalid())?;

        Ok(Realm(realm))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(
            HeaderValue::from_str(&self.0.to_string()).expect("realm value should be valid"),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(skip_all)]
pub async fn auth_middleware(
    State(state): State<ServerState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    bearer_auth: Option<TypedHeader<Authorization<Bearer>>>,
    token_auth: Option<TypedHeader<Authorization<Token>>>,
    basic_auth: Option<TypedHeader<Authorization<Basic>>>,
    realm: Option<TypedHeader<Realm>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let user = match (bearer_auth, token_auth, basic_auth, realm) {
        (Some(auth), None, None, None) => {
            let id = state
                .jwt_manager
                .get_claims(auth.token())
                .map_err(|err| {
                    tracing::error!(?err, "Failed to get claims from token");
                    Error::Unauthorized("Invalid token".to_string())
                })?
                .data
                .sub;

            logic::users::get_user(&state.pool, id).await?
        }
        (None, Some(auth), None, None) => {
            let raw_token = auth.0 .0 .0.clone();
            let Some((id, value)) = raw_token.split_once('_') else {
                return Err(Error::Unauthorized("Invalid API token format".to_string()));
            };

            let Ok(id) = id.parse::<i64>() else {
                return Err(Error::Unauthorized("Invalid API token content".to_string()));
            };

            let Some(token) = logic::api_tokens::get_api_token(&state.pool, id).await? else {
                return Err(Error::Unauthorized("API token unused".to_string()));
            };

            if !verify_password(value, &token.token)? {
                return Err(Error::Unauthorized("Invalid API token".to_string()));
            }

            logic::users::get_user(&state.pool, token.user_id).await?
        }
        (None, None, Some(auth), Some(realm)) => {
            let email = auth.username();
            let password = auth.password();
            let application_id = realm.0.into_inner();

            let Some(data) = logic::users::get_user_from_email_with_latest_password(
                &state.pool,
                email,
                application_id,
            )
            .await?
            else {
                return Err(Error::Unauthorized(
                    "User or application not found".to_string(),
                ));
            };

            if !verify_password(password, &data.password)? {
                return Err(Error::Unauthorized("Invalid API token".to_string()));
            }

            Some(data.user)
        }
        (None, None, Some(_), None) => return Err(Error::MissingHeader(REALM.to_string())),
        _ => return Err(Error::Unauthorized("Missing token".to_string())),
    };

    let Some(user) = user else {
        return Err(Error::Unauthorized("User does not exist".to_string()));
    };

    Span::current().record("user_id", user.id);

    if user.disabled {
        return Err(Error::Unauthorized("User is disabled".to_string()));
    }

    let action_log = models::action_logs::CreateActionLog {
        user_id: user.id,
        ip_address: addr.to_string(),
        user_agent: user_agent.to_string(),
        uri: request.uri().to_string(),
        method: request.method().to_string(),
    };

    request.extensions_mut().insert(user);

    tokio::task::spawn(async move {
        logic::action_logs::create_action_log(&state.pool, action_log)
            .await
            .unwrap_or_else(|err| {
                tracing::error!(%err, "Failed to create user log");
            });
    });

    Ok(next.run(request).await)
}
