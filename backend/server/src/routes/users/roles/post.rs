use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use database::{logic, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Assign a role to a user
#[utoipa::path(
    post,
    path = "",
    tag = "Users",
    responses(
        (status = 200, description = "Role assigned successfully", body = String),
        (status = 404, description = "Role doesn't exist")
    ),
    params(
        ("id", Path, description = "User database ID"),
        ("role_id", Path, description = "Role database ID")
    )
)]
pub async fn assign_user_role(
    State(pool): State<SqlitePool>,
    Path((id, role_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse> {
    let count = logic::roles::create_user_role(&pool, id, role_id).await?;

    if count == 0 {
        Err(Error::NotFound("Role not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
