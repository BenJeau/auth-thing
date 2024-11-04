use crate::models::providers::{ModifyProvider, Provider};
use sqlx::{Result, SqlitePool};

pub async fn get_provider(pool: &SqlitePool, id: i64) -> Result<Option<Provider>> {
    sqlx::query_as!(Provider, "SELECT * FROM providers WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_providers(pool: &SqlitePool) -> Result<Vec<Provider>> {
    sqlx::query_as!(Provider, "SELECT * FROM providers")
        .fetch_all(pool)
        .await
}

pub async fn create_provider(pool: &SqlitePool, provider: ModifyProvider) -> Result<i64> {
    sqlx::query_scalar!(
        "INSERT INTO providers (name, kind, client_id, client_secret, redirect_uri) VALUES (?, ?, ?, ?, ?) RETURNING id",
        provider.name,
        provider.kind,
        provider.client_id,
        provider.client_secret,
        provider.redirect_uri
    )
    .fetch_one(pool)
    .await
}

pub async fn update_provider(pool: &SqlitePool, id: i64, provider: ModifyProvider) -> Result<u64> {
    sqlx::query!(
        "UPDATE providers SET name = ?, kind = ?, client_id = ?, client_secret = ?, redirect_uri = ? WHERE id = ?",
        provider.name,
        provider.kind,
        provider.client_id,
        provider.client_secret,
        provider.redirect_uri,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn delete_provider(pool: &SqlitePool, id: i64) -> Result<u64> {
    sqlx::query!("DELETE FROM providers WHERE id = ?", id)
        .execute(pool)
        .await
        .map(|row| row.rows_affected())
}
