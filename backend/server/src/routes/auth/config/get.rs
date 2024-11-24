use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::logic;
use password::PasswordRequirementsBuilder;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{Error, Result, ServerState};

#[derive(Serialize, ToSchema)]
pub struct AuthConfigResponse {
    password_auth_enabled: bool,
    api_token_auth_enabled: bool,
    basic_auth_enabled: bool,
    password_requirements: password::PasswordRequirements,
}

/// Get authentication configuration for an application
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Successfully retrieved auth config", body = AuthConfigResponse),
        (status = 404, description = "Application not found"),
    ),
    tag = "auth"
)]
pub async fn auth_config(
    State(state): State<ServerState>,
    Path(application_slug): Path<String>,
) -> Result<impl IntoResponse> {
    let app = logic::applications::get_application_from_slug(&state.pool, &application_slug)
        .await?
        .ok_or(Error::NotFound("Application not found".to_string()))?;

    Ok(Json(AuthConfigResponse {
        password_auth_enabled: app.password_auth_enabled,
        api_token_auth_enabled: app.api_token_auth_enabled,
        basic_auth_enabled: app.basic_auth_enabled,
        password_requirements: PasswordRequirementsBuilder::from(&app).build()?,
    }))
}
