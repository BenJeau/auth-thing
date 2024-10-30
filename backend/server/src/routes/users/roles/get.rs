use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Get user roles
#[utoipa::path(
    get,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "List matching user roles by query", body = [models::roles::Role]),
    ),
    params(
        ("id", Path, description = "User database ID")
    )
)]
pub async fn get_user_roles(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    let roles = logic::roles::get_user_roles(&pool, id).await?;

    Ok(Json(roles))
}
