use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models::providers::MinimalProviderInfo};
use password::PasswordRequirementsBuilder;
use serde::Serialize;
use tokio::join;
use utoipa::ToSchema;

use crate::{Error, Result, ServerState};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfigResponse {
    password: PasswordConfig,
    api_token: ApiTokenConfig,
    basic: BasicConfig,
    providers: Vec<MinimalProviderInfo>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordConfig {
    enabled: bool,
    signup_enabled: bool,
    requirements: Option<password::PasswordRequirements>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenConfig {
    enabled: bool,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BasicConfig {
    enabled: bool,
}

/// Get authentication configuration for an application
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Successfully retrieved auth config", body = AuthConfigResponse),
        (status = 404, description = "Application not found"),
    ),
    params(
        ("slug", Path, description = "Application slug"),
    ),
    tag = "Auth"
)]
pub async fn auth_config(
    State(state): State<ServerState>,
    Path(application_slug): Path<String>,
) -> Result<impl IntoResponse> {
    let (app, providers) = join!(
        logic::applications::get_application_from_slug(&state.pool, &application_slug),
        logic::applications::get_application_providers_by_slug(&state.pool, &application_slug),
    );

    let app = app?.ok_or(Error::NotFound("Application not found".to_string()))?;
    let providers = providers?
        .into_iter()
        .map(MinimalProviderInfo::from)
        .collect();

    Ok(Json(AuthConfigResponse {
        password: PasswordConfig {
            enabled: app.password_auth_enabled,
            signup_enabled: app.password_auth_signup_enabled,
            requirements: app
                .password_auth_signup_enabled
                .then_some(PasswordRequirementsBuilder::from(&app).build()?),
        },
        api_token: ApiTokenConfig {
            enabled: app.api_token_auth_enabled,
        },
        basic: BasicConfig {
            enabled: app.basic_auth_enabled,
        },
        providers,
    }))
}
