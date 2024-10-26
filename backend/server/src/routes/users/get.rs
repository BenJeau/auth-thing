use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, SqlitePool};

use crate::Result;

/// Get user by database ID
#[utoipa::path(
    get,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "Get user by ID", body = database::models::users::User),
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
    let users = logic::users::get_user_by_id(&pool, id).await?;

    Ok(Json(users))
}

/// Get all users
#[utoipa::path(
    get,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "List matching users by query", body = [database::models::users::User]),
    )
)]
pub async fn get_users(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let users = logic::users::get_users(&pool).await?;

    Ok(Json(users))
}
