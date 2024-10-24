use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, SqlitePool};

use crate::Result;

pub async fn get_user(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let users = logic::users::get_user_by_id(&pool, id).await?;

    Ok(Json(users))
}

pub async fn get_users(State(pool): State<SqlitePool>) -> Result<impl IntoResponse> {
    let users = logic::users::get_users(&pool).await?;

    Ok(Json(users))
}
