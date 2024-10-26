use axum::{extract::State, response::IntoResponse, Json};
use database::{logic, models, SqlitePool};

use crate::Result;

/// Create a new user
#[utoipa::path(
    post,
    path = "",
    tag = "Users",
    request_body = database::models::users::ModifyUser,
    responses(
        (status = 200, description = "User created successfully", body = String),
        (status = 400, description = "User was not created")
    )
)]
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(user): Json<models::users::ModifyUser>,
) -> Result<impl IntoResponse> {
    let id = logic::users::create_user(&pool, user).await?;

    Ok(id.to_string())
}
