use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub picture: Option<String>,
    pub disabled: bool,
    pub verified: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyUser {
    pub email: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub picture: Option<String>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub verified: bool,
}
