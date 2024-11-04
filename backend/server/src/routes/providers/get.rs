use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::{Error, Result};

/// Get prodiver by database ID
#[utoipa::path(
    get,
    path = "",
    tag = "Providers",
    responses(
        (status = 200, description = "Get provider by ID", body = models::providers::Provider),
        (status = 404, description = "Provider was not found")
    ),
    params(
        ("id", Path, description = "Provider database ID")
    )
)]
pub async fn get_provider(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let provider = logic::providers::get_provider(&pool, id).await?;

    if let Some(provider) = provider {
        Ok(Json(provider))
    } else {
        Err(Error::NotFound(format!(
            "Provider with ID {} not found",
            id
        )))
    }
}

/// Get all providers
#[utoipa::path(
    get,
    path = "",
    tag = "Providers",
    responses(
        (status = 200, description = "List matching providers by query", body = [models::providers::Provider]),
    )
)]
pub async fn get_providers(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let providers = logic::providers::get_providers(&pool).await?;

    Ok(Json(providers))
}
