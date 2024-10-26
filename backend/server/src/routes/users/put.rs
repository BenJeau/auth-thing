use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Update a user
#[utoipa::path(
    put,
    path = "",
    tag = "Users",
    request_body = database::models::users::ModifyUser,
    responses(
        (status = 200, description = "User updated successfully", body = String),
        (status = 400, description = "User was not updated")
    )
)]
pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(user): Json<models::users::ModifyUser>,
) -> Result<impl IntoResponse> {
    let id = logic::users::update_user(&pool, id, user).await?;

    Ok(id.to_string())
}
