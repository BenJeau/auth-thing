use axum::{extract::State, response::IntoResponse, Json};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Create a new application
#[utoipa::path(
    post,
    path = "",
    tag = "Applications",
    request_body = models::applications::ModifyApplication,
    responses(
        (status = 200, description = "Application created successfully", body = String)
    )
)]
pub async fn create_application(
    State(pool): State<SqlitePool>,
    Json(user): Json<models::applications::ModifyApplication>,
) -> Result<impl IntoResponse> {
    let id = logic::applications::create_application(&pool, user).await?;

    Ok(id.to_string())
}
