use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct Role {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub creator_id: i64,
    pub application_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyRole {
    pub name: String,
    pub description: Option<String>,
}
