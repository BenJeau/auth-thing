use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Update a user
#[utoipa::path(
    put,
    path = "",
    tag = "Users",
    request_body = database::models::users::ModifyUser,
    responses(
        (status = 200, description = "User updated successfully", body = String),
        (status = 404, description = "User was not updated")
    )
)]
pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(user): Json<models::users::ModifyUser>,
) -> Result<impl IntoResponse> {
    let count = logic::users::update_user(&pool, id, user).await?;

    if count == 0 {
        Err(Error::NotFound("User not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
