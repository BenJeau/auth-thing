use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct Provider {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyProvider {
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}
