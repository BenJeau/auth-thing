use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, FromRow, Serialize, ToSchema, Zeroize, ZeroizeOnDrop)]
#[serde(rename_all = "camelCase")]
pub struct JwtConfig {
    #[zeroize(skip)]
    pub id: i64,
    #[zeroize(skip)]
    pub created_at: NaiveDateTime,
    #[zeroize(skip)]
    pub application_id: i64,
    #[zeroize(skip)]
    pub algorithm: String,
    #[serde(skip)]
    pub private_key: Vec<u8>,
    #[zeroize(skip)]
    pub public_key: Vec<u8>,
    #[zeroize(skip)]
    pub audience: Vec<String>,
    #[zeroize(skip)]
    pub expiration: i64,
    #[zeroize(skip)]
    pub issuer: String,
}

#[derive(Debug, FromRow, Zeroize, ZeroizeOnDrop)]
pub struct DbJwtConfig {
    #[zeroize(skip)]
    pub id: i64,
    #[zeroize(skip)]
    pub created_at: NaiveDateTime,
    #[zeroize(skip)]
    pub application_id: i64,
    #[zeroize(skip)]
    pub algorithm: String,
    pub private_key: Vec<u8>,
    #[zeroize(skip)]
    pub public_key: Vec<u8>,
    #[zeroize(skip)]
    pub audience: Vec<u8>,
    #[zeroize(skip)]
    pub expiration: i64,
    #[zeroize(skip)]
    pub issuer: String,
}

impl From<DbJwtConfig> for JwtConfig {
    fn from(db_jwt_config: DbJwtConfig) -> Self {
        let audience_string = std::str::from_utf8(&db_jwt_config.audience).unwrap();
        let audience = serde_json::from_str(audience_string).unwrap();

        JwtConfig {
            id: db_jwt_config.id,
            created_at: db_jwt_config.created_at,
            application_id: db_jwt_config.application_id,
            algorithm: db_jwt_config.algorithm.clone(),
            private_key: db_jwt_config.private_key.clone(),
            public_key: db_jwt_config.public_key.clone(),
            audience,
            expiration: db_jwt_config.expiration,
            issuer: db_jwt_config.issuer.clone(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModifyJwtConfig {
    pub algorithm: String,
    pub audience: Vec<String>,
    pub expiration: i64,
}
