use crate::models::users::{InnerModifyUser, ModifyUser, User, UserWithLatestPassword};
use chrono::NaiveDateTime;
use sqlx::{Result, SqlitePool};

pub async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>> {
    sqlx::query_as!(User, "SELECT id, created_at, updated_at, email, name, username, picture, disabled, email_verified, two_factor_enabled, preferred_locale FROM users WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_from_email_with_latest_password(
    pool: &SqlitePool,
    email: &str,
    application_id: i64,
) -> Result<Option<UserWithLatestPassword>> {
    sqlx::query_as(
        "SELECT users.id, users.created_at, users.updated_at, users.email, users.name, users.username, users.picture, users.disabled, users.email_verified, users.two_factor_enabled, users.preferred_locale, password FROM users INNER JOIN application_passwords ON users.id = application_passwords.user_id INNER JOIN applications ON applications.id = ? WHERE users.email = ? ORDER BY users.created_at DESC"
    )
    .bind(application_id)
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_from_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
    sqlx::query_as!(User, "SELECT id, created_at, updated_at, email, name, username, picture, disabled, email_verified, two_factor_enabled, preferred_locale FROM users WHERE email = ?", email)
        .fetch_optional(pool)
        .await
}

pub async fn get_users(pool: &SqlitePool) -> Result<Vec<User>> {
    sqlx::query_as!(User, "SELECT id, created_at, updated_at, email, name, username, picture, disabled, email_verified, two_factor_enabled, preferred_locale FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_users_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT users.id, users.created_at, users.updated_at, users.email, users.name, users.username, users.picture, users.disabled, users.email_verified, users.two_factor_enabled, users.preferred_locale FROM users INNER JOIN users_roles ON users.id = users_roles.user_id INNER JOIN roles ON users_roles.role_id = roles.id WHERE roles.application_id = ?",
        application_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_user(pool: &SqlitePool, user: &InnerModifyUser) -> Result<i64> {
    sqlx::query_scalar!(
        "INSERT INTO users (email, name, username, picture, disabled, email_verified, verification_code, verification_code_created_at, two_factor_enabled, two_factor_secret) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
        user.modify_user.email,
        user.modify_user.name,
        user.modify_user.username,
        user.modify_user.picture,
        user.modify_user.disabled,
        user.email_verified,
        user.verification_code,
        user.verification_code_created_at,
        user.two_factor_enabled,
        user.two_factor_secret,
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(pool: &SqlitePool, id: i64, user: ModifyUser) -> Result<u64> {
    sqlx::query!(
        "UPDATE users SET email = ?, name = ?, username = ?, picture = ?, disabled = ? WHERE id = ?",
        user.email,
        user.name,
        user.username,
        user.picture,
        user.disabled,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn update_user_verification_code(
    pool: &SqlitePool,
    id: i64,
    verification_code: String,
    verification_code_created_at: NaiveDateTime,
) -> Result<u64> {
    sqlx::query!(
        "UPDATE users SET verification_code = ?, verification_code_created_at = ? WHERE id = ?",
        verification_code,
        verification_code_created_at,
        id
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn get_two_factor_secret(pool: &SqlitePool, user_id: i64) -> Result<Option<String>> {
    sqlx::query_scalar!("SELECT two_factor_secret FROM users WHERE id = ?", user_id)
        .fetch_one(pool)
        .await
}

pub async fn verify_email(pool: &SqlitePool, user_id: i64, token: &str) -> Result<bool> {
    sqlx::query_scalar!(
        "UPDATE users SET email_verified = true WHERE id = ? AND verification_code = ?",
        user_id,
        token
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected() > 0)
}
