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
    tag = "Applications",
    responses(
        (status = 201, description = "Application deleted successfully"),
        (status = 404, description = "Application not found"),
    ),
    params(
        ("id", Path, description = "Application database ID")
    )
)]
pub async fn delete_application(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let count = logic::applications::delete_application(&pool, id).await?;

    if count == 0 {
        Err(Error::NotFound("Application not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
