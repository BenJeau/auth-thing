use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::error::Result;

/// Verifies if the backend is healthy and it's related services
#[utoipa::path(
    get,
    path = "",
    tag = "Health",
    responses(
        (status = 200, description = "Backend is healthy", example = json!({"status": "ok"}))
    )
)]
pub async fn health() -> Result<impl IntoResponse> {
    Ok(Json(json!({"status": "ok"})))
}
