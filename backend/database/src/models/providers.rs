use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyProvider {
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
}

#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MinimalProviderInfo {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub auth_url: String,
}

impl From<Provider> for MinimalProviderInfo {
    fn from(provider: Provider) -> Self {
        MinimalProviderInfo {
            id: provider.id,
            name: provider.name,
            kind: provider.kind,
            client_id: provider.client_id,
            redirect_uri: provider.redirect_uri,
            auth_url: provider.auth_url,
        }
    }
}
