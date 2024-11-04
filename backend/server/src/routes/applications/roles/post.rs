use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Create a new role
#[utoipa::path(
    post,
    path = "",
    tag = "Roles",
    request_body = models::roles::ModifyRole,
    responses(
        (status = 200, description = "Role created successfully", body = String)
    ),
    params(
        ("id", Path, description = "Application database ID")
    )
)]
pub async fn create_role(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(role): Json<models::roles::ModifyRole>,
) -> Result<impl IntoResponse> {
    let id = logic::roles::create_role(&pool, role, 1, id).await?;

    Ok(id.to_string())
}
