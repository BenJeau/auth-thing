use crate::models::roles::{ModifyRole, Role};
use sqlx::{Result, SqlitePool};

pub async fn get_roles(pool: &SqlitePool) -> Result<Vec<Role>> {
    sqlx::query_as!(Role, "SELECT * FROM roles",)
        .fetch_all(pool)
        .await
}

pub async fn get_role(pool: &SqlitePool, id: i64) -> Result<Option<Role>> {
    sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_application_roles(pool: &SqlitePool, application_id: i64) -> Result<Vec<Role>> {
    sqlx::query_as!(
        Role,
        "SELECT * FROM roles WHERE application_id = ?",
        application_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_role(
    pool: &SqlitePool,
    role: ModifyRole,
    creator_id: i64,
    application_id: i64,
) -> Result<i64> {
    sqlx::query_scalar!(
        "INSERT INTO roles (creator_id, application_id, name, description) VALUES (?, ?, ?, ?) RETURNING id",
        creator_id,
        application_id,
        role.name,
        role.description
    )
    .fetch_one(pool)
    .await
}

pub async fn update_role(pool: &SqlitePool, id: i64, role: ModifyRole) -> Result<u64> {
    sqlx::query!(
        "UPDATE roles SET name = ?, description = ? WHERE id = ?",
        role.name,
        role.description,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn delete_role(pool: &SqlitePool, id: i64) -> Result<u64> {
    sqlx::query!("DELETE FROM roles WHERE id = ?", id)
        .execute(pool)
        .await
        .map(|row| row.rows_affected())
}
