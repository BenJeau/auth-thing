use crate::models::jwt_configs::{DbJwtConfig, JwtConfig, ModifyJwtConfig};
use sqlx::{Result, SqlitePool};

pub async fn create_jwt_config(
    pool: &SqlitePool,
    application_id: i64,
    jwt_config: &ModifyJwtConfig,
    private_key: &[u8],
    public_key: &[u8],
) -> Result<i64> {
    let audience = serde_json::to_value(jwt_config.audience.clone()).unwrap();

    sqlx::query_scalar!(
        "INSERT INTO jwt_configs (application_id, algorithm, private_key, public_key, audience, expiration) VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        application_id,
        jwt_config.algorithm,
        private_key,
        public_key,
        audience,
        jwt_config.expiration
    )
    .fetch_one(pool)
    .await
}

pub async fn get_jwt_configs(pool: &SqlitePool, application_id: i64) -> Result<Vec<JwtConfig>> {
    Ok(sqlx::query_as!(
        DbJwtConfig,
        "SELECT jwt_configs.*, applications.name AS issuer FROM jwt_configs INNER JOIN applications ON jwt_configs.application_id = applications.id WHERE application_id = ?",
        application_id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(Into::into)
    .collect())
}

pub async fn get_active_jwt_config(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Option<JwtConfig>> {
    Ok(sqlx::query_as!(
        DbJwtConfig,
        "SELECT jwt_configs.*, applications.name AS issuer FROM jwt_configs INNER JOIN applications ON jwt_configs.application_id = applications.id WHERE application_id = ? AND jwt_configs.id = applications.active_jwt_config_id",
        application_id
    )
    .fetch_optional(pool)
    .await?
    .map(Into::into))
}
