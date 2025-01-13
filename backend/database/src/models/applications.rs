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
    pub api_token_auth_enabled: bool,
    pub basic_auth_enabled: bool,
    pub password_auth_enabled: bool,
    pub password_auth_signup_enabled: bool,
    pub password_min_length: i64,
    pub password_max_length: i64,
    pub password_min_lowercase: i64,
    pub password_min_uppercase: i64,
    pub password_min_number: i64,
    pub password_min_special: i64,
    pub password_unique: bool,
    pub password_min_strength: String,
    pub verification_required: bool,
    pub verification_method: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyApplication {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub icon: Option<String>,
}

impl From<&Application> for password::PasswordRequirementsBuilder {
    fn from(value: &Application) -> Self {
        password::PasswordRequirementsBuilder::new()
            .min(value.password_min_length.max(0) as usize)
            .max(value.password_max_length.max(0) as usize)
            .min_lowercase(value.password_min_lowercase.max(0) as usize)
            .min_uppercase(value.password_min_uppercase.max(0) as usize)
            .min_number(value.password_min_number.max(0) as usize)
            .min_special(value.password_min_special.max(0) as usize)
            .unique(value.password_unique)
            .min_strength(value.password_min_strength.as_str().into())
    }
}
