use axum::{extract::State, response::IntoResponse, Json};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Create a new provider
#[utoipa::path(
    post,
    path = "",
    tag = "Providers",
    request_body = models::providers::ModifyProvider,
    responses(
        (status = 200, description = "Provider created successfully", body = String)
    )
)]
pub async fn create_provider(
    State(pool): State<SqlitePool>,
    Json(provider): Json<models::providers::ModifyProvider>,
) -> Result<impl IntoResponse> {
    let id = logic::providers::create_provider(&pool, provider).await?;

    Ok(id.to_string())
}
