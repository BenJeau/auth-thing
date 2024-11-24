// use auth::openid::parse_and_validate_referer;
use axum::{
    extract::{ConnectInfo, OriginalUri, Path, State},
    // http::HeaderMap,
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use database::{logic, models};
use http::StatusCode;
use password::hash_password;
use std::net::SocketAddr;
use tokio::join;
use tracing::warn;

use crate::{schemas::SignupUserRequest, Error, Result, ServerState};

/// Signup to create a new user
#[utoipa::path(
    post,
    path = "",
    tag = "Auth",
    responses(
        (status = 201, description = "User created successfully"),
    ),
    params(
        ("slug", Path, description = "Application slug"),
    ),
    request_body(
        description = "User information needed to create new user",
        content_type = "application/json",
        content = SignupUserRequest
    )
)]
pub async fn signup(
    State(state): State<ServerState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    OriginalUri(uri): OriginalUri,
    // headers: HeaderMap,
    Path(application_slug): Path<String>,
    Json(data): Json<SignupUserRequest>,
) -> Result<impl IntoResponse> {
    let Some(application_id) =
        logic::applications::get_application_id(&state.pool, &application_slug).await?
    else {
        return Err(Error::NotFound("Application not found".to_string()));
    };

    let (verification_code, hashed_verification_code) = if state.mailer.is_some() {
        let code = state
            .crypto
            .generate_random_numeric_string(state.config.auth.email.code_length);
        let hashed_code = hash_password(&code)?;
        (Some(code), Some(hashed_code))
    } else {
        (None, None)
    };

    let verification_code_created_at = if state.mailer.is_some() {
        Some(chrono::Utc::now().naive_utc())
    } else {
        None
    };

    let create_user = models::users::InnerModifyUser {
        modify_user: models::users::ModifyUser {
            email: data.email.clone(),
            name: data.name,
            username: data.username,
            picture: None,
            disabled: false,
        },
        email_verified: state.mailer.is_none(),
        verification_code: hashed_verification_code,
        verification_code_created_at,
        two_factor_enabled: false,
        two_factor_secret: None,
    };
    let user_id = logic::users::create_user(&state.pool, &create_user).await?;

    let action_log = models::action_logs::CreateActionLog {
        user_id,
        ip_address: addr.to_string(),
        user_agent: user_agent.to_string(),
        uri: uri.to_string(),
        method: "GET".to_string(),
    };

    let maybe_send_email = async {
        if let Some(mailer) = state.mailer {
            crate::emails::send_verification_email(
                &verification_code.unwrap(),
                &create_user.modify_user.email,
                &mailer,
            )
            .await?;
        } else {
            warn!("No mailer configured, skipping email confirmation, defaulting to verifying the user");
        }

        Result::Ok(())
    };

    let (result, action_log, application) = join!(
        maybe_send_email,
        logic::action_logs::create_action_log(&state.pool, action_log),
        logic::applications::create_application_password(
            &state.pool,
            application_id,
            user_id,
            hash_password(&data.password)?,
        )
    );

    result?;
    action_log?;
    application?;

    // parse_and_validate_referer(&headers, &state.config.auth)?;

    Ok(StatusCode::CREATED)
}
