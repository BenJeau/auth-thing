use crate::models::api_tokens::ApiToken;
use sqlx::{Result, SqlitePool};

pub async fn get_api_token(pool: &SqlitePool, id: i64) -> Result<Option<ApiToken>> {
    sqlx::query_as!(ApiToken, "SELECT * FROM api_tokens WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}
