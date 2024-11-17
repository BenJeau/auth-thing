use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// A user of the platform
#[derive(Debug, FromRow, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Database ID of the user
    pub id: i64,
    /// Time when the user was created
    pub created_at: NaiveDateTime,
    /// Time when the user was updated
    pub updated_at: NaiveDateTime,
    /// Email of the user
    pub email: String,
    /// Full name of the user
    pub name: Option<String>,
    /// Username of the user
    pub username: Option<String>,
    /// Picture of the user
    pub picture: Option<String>,
    /// Whether the user is enabled or not, if they are able to login/access the platform
    pub disabled: bool,
    /// Language and general location (locale) of the user
    pub preferred_locale: Option<String>,
    /// Whether the user has verified their email address
    pub email_verified: bool,
    /// Whether two-factor authentication is enabled for this user
    pub two_factor_enabled: bool,
}

#[derive(FromRow)]

pub struct UserWithLatestPassword {
    #[sqlx(flatten)]
    pub user: User,
    pub password: String,
}

/// Fields to modify a user
#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyUser {
    /// Email of the user
    pub email: String,
    /// Full name of the user
    pub name: Option<String>,
    /// Username of the user
    pub username: Option<String>,
    /// Picture of the user
    pub picture: Option<String>,
    /// Whether the user is enabled or not, if they are able to login/access the platform
    #[serde(default)]
    pub disabled: bool,
}

pub struct InnerModifyUser {
    pub email_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_created_at: Option<NaiveDateTime>,
    pub two_factor_enabled: bool,
    pub two_factor_secret: Option<String>,
    pub modify_user: ModifyUser,
}
