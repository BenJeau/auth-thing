use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use database::{logic, SqlitePool};
use http::StatusCode;

use crate::{Error, Result};

/// Delete a provider
#[utoipa::path(
    delete,
    path = "",
    tag = "Providers",
    responses(
        (status = 201, description = "Provider deleted successfully"),
        (status = 404, description = "Provider not found"),
    ),
    params(
        ("id", Path, description = "Provider database ID")
    )
)]
pub async fn delete_provider(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let count = logic::providers::delete_provider(&pool, id).await?;

    if count == 0 {
        Err(Error::NotFound("Provider not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
