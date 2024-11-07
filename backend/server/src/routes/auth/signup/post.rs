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
use std::net::SocketAddr;
use tokio::join;

use crate::{crypto::hash_password, schemas::SignupUserRequest, Error, Result, ServerState};

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

    let create_user = models::users::ModifyUser {
        email: data.email,
        name: data.name,
        username: data.username,
        picture: None,
        disabled: false,
        verified: false,
    };
    let user_id = logic::users::create_user(&state.pool, create_user).await?;

    let action_log = models::action_logs::CreateActionLog {
        user_id,
        ip_address: addr.to_string(),
        user_agent: user_agent.to_string(),
        uri: uri.to_string(),
        method: "GET".to_string(),
    };

    let (action_log, application) = join!(
        logic::action_logs::create_action_log(&state.pool, action_log),
        logic::applications::create_application_password(
            &state.pool,
            application_id,
            user_id,
            hash_password(&data.password)?,
        )
    );

    action_log?;
    application?;

    // parse_and_validate_referer(&headers, &state.config.auth)?;

    Ok(StatusCode::CREATED)
}
