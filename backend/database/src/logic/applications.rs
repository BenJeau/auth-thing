use crate::{
    models::{
        applications::{Application, ModifyApplication},
        providers::Provider,
    },
    slug::slugify,
};
use sqlx::{Result, SqlitePool};

pub async fn get_application(pool: &SqlitePool, id: i64) -> Result<Option<Application>> {
    sqlx::query_as!(Application, "SELECT id, created_at, updated_at, creator_id, slug, name, description, website, icon, api_token_auth_enabled, basic_auth_enabled, password_auth_enabled, password_auth_signup_enabled, password_min_length, password_max_length, password_min_lowercase, password_min_uppercase, password_min_number, password_min_special, password_unique, password_min_strength, verification_required, verification_method, verification_code FROM applications WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_applications(pool: &SqlitePool) -> Result<Vec<Application>> {
    sqlx::query_as!(
        Application,
        "SELECT id, created_at, updated_at, creator_id, slug, name, description, website, icon, api_token_auth_enabled, basic_auth_enabled, password_auth_enabled, password_auth_signup_enabled, password_min_length, password_max_length, password_min_lowercase, password_min_uppercase, password_min_number, password_min_special, password_unique, password_min_strength, verification_required, verification_method, verification_code FROM applications ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_application_id(pool: &SqlitePool, slug: &str) -> Result<Option<i64>> {
    sqlx::query_scalar!("SELECT id FROM applications WHERE slug = ?", slug)
        .fetch_optional(pool)
        .await
}

pub async fn get_application_from_slug(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<Application>> {
    sqlx::query_as!(
        Application,
        "SELECT id, created_at, updated_at, creator_id, slug, name, description, website, icon, api_token_auth_enabled, basic_auth_enabled, password_auth_enabled, password_auth_signup_enabled, password_min_length, password_max_length, password_min_lowercase, password_min_uppercase, password_min_number, password_min_special, password_unique, password_min_strength, verification_required, verification_method, verification_code FROM applications WHERE slug = ?",
        slug
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_application(pool: &SqlitePool, application: ModifyApplication) -> Result<i64> {
    let slug = slugify(&application.name);

    sqlx::query_scalar!(
        "INSERT INTO applications (slug, name, description, website, icon, creator_id) VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        slug,
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

pub async fn create_application_password(
    pool: &SqlitePool,
    application_id: i64,
    user_id: i64,
    hashed_password: String,
) -> Result<u64> {
    sqlx::query!(
        "INSERT INTO application_passwords (application_id, user_id, password) VALUES (?, ?, ?)",
        application_id,
        user_id,
        hashed_password,
    )
    .execute(pool)
    .await
    .map(|row| row.rows_affected())
}

pub async fn get_application_providers_by_slug(
    pool: &SqlitePool,
    application_slug: &str,
) -> Result<Vec<Provider>> {
    sqlx::query_as!(
        Provider,
        "SELECT providers.* FROM application_providers JOIN providers ON application_providers.provider_id = providers.id WHERE application_providers.application_id = (SELECT id FROM applications WHERE slug = ?)",
        application_slug
    )
    .fetch_all(pool)
    .await
}
