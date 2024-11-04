use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Get roles for application
#[utoipa::path(
    get,
    path = "",
    tag = "Roles",
    responses(
        (status = 200, description = "List matching roles by query", body = [models::roles::Role]),
    ),
    params(
        ("id", Path, description = "Application database ID")
    )
)]
pub async fn get_application_roles(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    let roles = logic::roles::get_application_roles(&pool, id).await?;

    Ok(Json(roles))
}
