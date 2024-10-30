use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::{Error, Result};

/// Get user by database ID
#[utoipa::path(
    get,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "Get user by ID", body = models::users::User),
        (status = 404, description = "User was not found")
    ),
    params(
        ("id", Path, description = "User database ID")
    )
)]
pub async fn get_user(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let users = logic::users::get_user(&pool, id).await?;

    if let Some(user) = users {
        Ok(Json(user))
    } else {
        Err(Error::NotFound(format!("User with ID {} not found", id)))
    }
}

/// Get all users
#[utoipa::path(
    get,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "List matching users by query", body = [models::users::User]),
    )
)]
pub async fn get_users(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let users = logic::users::get_users(&pool).await?;

    Ok(Json(users))
}
