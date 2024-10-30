use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::{Error, Result};

/// Get role by database ID
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Get role by ID", body = models::roles::Role),
        (status = 404, description = "Role was not found")
    ),
    params(
        ("id", Path, description = "Role database ID")
    )
)]
pub async fn get_role(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let role = logic::roles::get_role(&pool, id).await?;

    if let Some(role) = role {
        Ok(Json(role))
    } else {
        Err(Error::NotFound(format!("Role with ID {} not found", id)))
    }
}

/// Get all roles
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List matching roles by query", body = [models::roles::Role]),
    )
)]
pub async fn get_roles(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let roles = logic::roles::get_roles(&pool).await?;

    Ok(Json(roles))
}
