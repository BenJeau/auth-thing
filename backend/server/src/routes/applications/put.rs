use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Update an application
#[utoipa::path(
    put,
    path = "",
    tag = "Applications",
    request_body = database::models::applications::ModifyApplication,
    responses(
        (status = 200, description = "Application updated successfully", body = String),
        (status = 400, description = "Application was not updated")
    )
)]
pub async fn update_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(application): Json<models::applications::ModifyApplication>,
) -> Result<impl IntoResponse> {
    let id = logic::applications::update_application(&pool, id, application).await?;

    Ok(id.to_string())
}
