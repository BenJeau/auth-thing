use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Update an application
#[utoipa::path(
    put,
    path = "",
    tag = "Applications",
    request_body = models::applications::ModifyApplication,
    responses(
        (status = 200, description = "Application updated successfully", body = String),
        (status = 404, description = "Application was not updated")
    )
)]
pub async fn update_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(application): Json<models::applications::ModifyApplication>,
) -> Result<impl IntoResponse> {
    let count = logic::applications::update_application(&pool, id, application).await?;

    if count == 0 {
        Err(Error::NotFound("Application not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
