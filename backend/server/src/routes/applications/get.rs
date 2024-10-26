use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, SqlitePool};

use crate::Result;

/// Get application by database ID
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Get application by ID", body = database::models::applications::Application),
        (status = 404, description = "Application was not found")
    ),
    params(
        ("id", Path, description = "Application database ID")
    )
)]
pub async fn get_application(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let application = logic::applications::get_application_by_id(&pool, id).await?;

    Ok(Json(application))
}

/// Get all applications
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List matching applications by query", body = [database::models::applications::Application]),
    )
)]
pub async fn get_applications(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let applications = logic::applications::get_applications(&pool).await?;

    Ok(Json(applications))
}
