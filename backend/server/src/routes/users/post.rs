use axum::{extract::State, response::IntoResponse, Json};
use database::{logic, models, SqlitePool};

use crate::Result;

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(user): Json<models::users::ModifyUser>,
) -> Result<impl IntoResponse> {
    let id = logic::users::create_user(&pool, user).await?;

    Ok(id.to_string())
}
