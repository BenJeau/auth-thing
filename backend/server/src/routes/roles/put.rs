use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Update a role
#[utoipa::path(
    put,
    path = "",
    tag = "Roles",
    request_body = models::roles::ModifyRole,
    responses(
        (status = 200, description = "Role updated successfully", body = String),
        (status = 404, description = "Role was not updated")
    ),
    params(
        ("id", Path, description = "Role database ID")
    )
)]
pub async fn update_role(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(role): Json<models::roles::ModifyRole>,
) -> Result<impl IntoResponse> {
    let count = logic::roles::update_role(&pool, id, role).await?;

    if count == 0 {
        Err(Error::NotFound("Role not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
