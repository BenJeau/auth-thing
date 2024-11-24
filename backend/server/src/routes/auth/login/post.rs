use axum::{
    extract::{ConnectInfo, OriginalUri, Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use database::{logic, models};
use password::verify_password;
use std::net::SocketAddr;

use crate::{
    jwt::CreateClaims,
    schemas::{LoginUserRequest, LoginUserResponse},
    Error, Result, ServerState,
};

/// Login to authenticate and generate a new JWT
#[utoipa::path(
    post,
    path = "",
    tag = "Auth",
    responses(
        (status = 200, description = "User logged in successfully", body = LoginUserResponse),
    ),
    params(
        ("slug", Path, description = "Application slug"),
    ),
    request_body(
        description = "User information needed to authenticate a user",
        content_type = "application/json",
        content = LoginUserRequest
    )
)]
pub async fn login(
    State(state): State<ServerState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    OriginalUri(uri): OriginalUri,
    Path(application_slug): Path<String>,
    // headers: HeaderMap,
    Json(data): Json<LoginUserRequest>,
) -> Result<impl IntoResponse> {
    let Some(application_id) =
        logic::applications::get_application_id(&state.pool, &application_slug).await?
    else {
        return Err(Error::NotFound("Application not found".to_string()));
    };

    let Some(user) = logic::users::get_user_from_email_with_latest_password(
        &state.pool,
        &data.email,
        application_id,
    )
    .await?
    else {
        return Err(Error::InvalidCredentials);
    };

    if !verify_password(&data.password, &user.password)? {
        return Err(Error::InvalidCredentials);
    }

    if user.user.disabled {
        return Err(Error::DisabledUser);
    }

    let action_log = models::action_logs::CreateActionLog {
        user_id: user.user.id,
        ip_address: addr.to_string(),
        user_agent: user_agent.to_string(),
        uri: uri.to_string(),
        method: "POST".to_string(),
    };
    logic::action_logs::create_action_log(&state.pool, action_log).await?;

    let claims = CreateClaims {
        sub: user.user.id,
        email: user.user.email,
        email_verified: Some(user.user.email_verified),
        email_code_created_at: user.user.verification_code_created_at,
        name: user.user.name,
        given_name: None,
        family_name: None,
        locale: user.user.preferred_locale,
        roles: vec![],
        provider: "this thing".to_string(),
    };

    let jwt_token = state.jwt_manager.generate_jwt(claims)?;

    Ok(Json(LoginUserResponse { jwt_token }).into_response())
}
