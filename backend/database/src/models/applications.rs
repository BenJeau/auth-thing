use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub creator_id: i64,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub icon: Option<String>,
    pub password_auth: bool,
    pub password_min_length: i64,
    pub password_max_length: Option<i64>,
    pub password_requires_lowercase: bool,
    pub password_requires_uppercase: bool,
    pub password_requires_number: bool,
    pub password_requires_special: bool,
    pub password_requires_unique: bool,
    pub password_requires_non_common: bool,
    pub verification_required: bool,
    pub verification_method: Option<String>,
    pub verification_code: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyApplication {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub icon: Option<String>,
}
