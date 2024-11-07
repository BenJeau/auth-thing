use crate::models::action_logs::{ActionLog, CreateActionLog};
use sqlx::{Result, SqlitePool};

pub async fn get_action_logs(pool: &SqlitePool) -> Result<Vec<ActionLog>> {
    sqlx::query_as!(ActionLog, "SELECT * FROM action_logs")
        .fetch_all(pool)
        .await
}

pub async fn create_action_log(pool: &SqlitePool, log: CreateActionLog) -> Result<()> {
    sqlx::query!(
        "INSERT INTO action_logs (user_id, ip_address, user_agent, uri, method) VALUES (?, ?, ?, ?, ?)",
        log.user_id,
        log.ip_address,
        log.user_agent,
        log.uri,
        log.method
    )
    .execute(pool)
    .await?;

    Ok(())
}
