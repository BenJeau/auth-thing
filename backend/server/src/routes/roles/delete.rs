use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use database::{logic, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

#[utoipa::path(
    delete,
    path = "",
    tag = "Roles",
    responses(
        (status = 201, description = "Role deleted successfully"),
        (status = 404, description = "Role not found"),
    ),
    params(
        ("id", Path, description = "Role database ID")
    )
)]
pub async fn delete_role(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let count = logic::roles::delete_role(&pool, id).await?;

    if count == 0 {
        Err(Error::NotFound("Role not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
