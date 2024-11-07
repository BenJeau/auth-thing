use crate::models::users::{ModifyUser, User, UserWithLatestPassword};
use sqlx::{Result, SqlitePool};

pub async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_from_email_with_latest_password(
    pool: &SqlitePool,
    email: &str,
    application_id: i64,
) -> Result<Option<UserWithLatestPassword>> {
    sqlx::query_as(
        "SELECT users.*, password FROM users INNER JOIN application_passwords ON users.id = application_passwords.user_id INNER JOIN applications ON applications.id = ? WHERE users.email = ? ORDER BY users.created_at DESC"
    )
    .bind(application_id)
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_from_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(pool)
        .await
}

pub async fn get_users(pool: &SqlitePool) -> Result<Vec<User>> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_users_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT users.* FROM users INNER JOIN users_roles ON users.id = users_roles.user_id INNER JOIN roles ON users_roles.role_id = roles.id WHERE roles.application_id = ?",
        application_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_user(pool: &SqlitePool, user: ModifyUser) -> Result<i64> {
    sqlx::query_scalar!(
        "INSERT INTO users (email, name, username, picture, disabled, verified) VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        user.email,
        user.name,
        user.username,
        user.picture,
        user.disabled,
        user.verified
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(pool: &SqlitePool, id: i64, user: ModifyUser) -> Result<u64> {
    sqlx::query!(
        "UPDATE users SET email = ?, name = ?, username = ?, picture = ?, disabled = ?, verified = ? WHERE id = ?",
        user.email,
        user.name,
        user.username,
        user.picture,
        user.disabled,
        user.verified,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}
