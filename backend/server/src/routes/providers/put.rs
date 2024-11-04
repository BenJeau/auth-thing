use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Update a provider
#[utoipa::path(
    put,
    path = "",
    tag = "Providers",
    request_body = models::providers::ModifyProvider,
    responses(
        (status = 200, description = "Provider updated successfully", body = String),
        (status = 404, description = "Provider was not updated")
    )
)]
pub async fn update_provider(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(provider): Json<models::providers::ModifyProvider>,
) -> Result<impl IntoResponse> {
    let count = logic::providers::update_provider(&pool, id, provider).await?;

    if count == 0 {
        Err(Error::NotFound("Provider not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
