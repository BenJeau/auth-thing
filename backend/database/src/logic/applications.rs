use crate::models::applications::{Application, ModifyApplication};
use sqlx::{Result, SqlitePool};

pub async fn get_application_by_id(pool: &SqlitePool, id: i64) -> Result<Application> {
    sqlx::query_as!(Application, "SELECT * FROM applications WHERE id = ?", id)
        .fetch_one(pool)
        .await
}

pub async fn get_applications(pool: &SqlitePool) -> Result<Vec<Application>> {
    sqlx::query_as!(
        Application,
        "SELECT * FROM applications ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn create_application(pool: &SqlitePool, application: ModifyApplication) -> Result<i64> {
    sqlx::query_scalar!(
        "INSERT INTO applications (name, description, website, icon, creator_id) VALUES (?, ?, ?, ?, ?) RETURNING id",
        application.name,
        application.description,
        application.website,
        application.icon,
        1 // TODO: get creator ID from context
    )
    .fetch_one(pool)
    .await
}

pub async fn update_application(
    pool: &SqlitePool,
    id: i64,
    application: ModifyApplication,
) -> Result<u64> {
    sqlx::query!(
        "UPDATE applications SET name = ?, description = ?, website = ?, icon = ? WHERE id = ?",
        application.name,
        application.description,
        application.website,
        application.icon,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn delete_application(pool: &SqlitePool, id: i64) -> Result<u64> {
    sqlx::query!("DELETE FROM applications WHERE id = ?", id)
        .execute(pool)
        .await
        .map(|row| row.rows_affected())
}
