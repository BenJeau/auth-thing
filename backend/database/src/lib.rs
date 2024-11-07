pub use sqlx::{migrate::MigrateError, Error, SqlitePool};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions,
};
use std::str::FromStr;
use tracing::info;

pub mod logic;
pub mod models;
mod slug;

pub async fn connect_to_db(
    url: &str,
    max_connections: u32,
    min_connections: u32,
) -> Result<SqlitePool, Error> {
    assert!(max_connections >= min_connections);

    info!("Connecting to database");
    let options = SqliteConnectOptions::from_str(url)?;

    SqlitePoolOptions::new()
        .max_connections(75)
        .min_connections(5)
        .connect_with(options.disable_statement_logging())
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), MigrateError> {
    info!("Running migrations");
    sqlx::migrate!("./migrations").run(pool).await
}
