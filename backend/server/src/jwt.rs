use chrono::NaiveDateTime;
use database::{models::jwt_configs::ModifyJwtConfig, SqlitePool};
use jwt::{Algorithm, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::{crypto, Error, Result};

pub async fn jwt_validation(
    pool: &SqlitePool,
    crypto: &crypto::Crypto,
    token: &str,
) -> Result<Claims> {
    let application_id = extract_application_id_from_token(token, pool).await?;

    let jwt_configs = database::logic::jwt_configs::get_jwt_configs(pool, application_id).await?;

    for config in jwt_configs {
        let algorithm = jwt::Algorithm::try_from(config.algorithm.clone())?;
        let decrypted_public_key = crypto.decrypt(&config.public_key)?;

        let Ok(claims) = algorithm.decode_jwt(
            &decrypted_public_key,
            &[config.issuer.clone()],
            &config.audience,
            token,
        ) else {
            continue;
        };

        return Ok(claims);
    }

    Err(Error::UnableToValidateToken)
}

async fn extract_application_id_from_token(token: &str, pool: &SqlitePool) -> Result<i64> {
    let claims = jwt::get_claims_without_validation::<Claims>(token)?;

    let application_id = database::logic::applications::get_application_id(pool, &claims.iss)
        .await?
        .ok_or(Error::UnableToValidateToken)?;

    Ok(application_id)
}

async fn generate_keys(
    application_id: i64,
    algorithm: Algorithm,
    pool: &SqlitePool,
    jwt_config: &ModifyJwtConfig,
    crypto: &crypto::Crypto,
) -> Result<(EncodingKey, DecodingKey)> {
    let keys = algorithm.generate_keys()?;

    database::logic::jwt_configs::create_jwt_config(
        pool,
        application_id,
        jwt_config,
        &crypto.encrypt(&keys.private_key)?,
        &crypto.encrypt(&keys.public_key)?,
    )
    .await?;

    Ok((
        algorithm.jwt_encoding_key(&keys.private_key),
        algorithm.jwt_decoding_key(&keys.public_key),
    ))
}

pub fn generate_jwt(
    claims: CreateClaims,
    config: database::models::jwt_configs::JwtConfig,
    crypto: &crypto::Crypto,
) -> Result<String> {
    let all_claims = Claims::new(claims, &config.issuer, &config.audience, config.expiration);
    let decrypted_private_key = crypto.decrypt(&config.private_key)?;
    let algorithm = jwt::Algorithm::try_from(config.algorithm.clone())?;
    let token = algorithm.encode_jwt(&decrypted_private_key, &all_claims)?;

    Ok(token)
}

/// JWT claims
#[derive(Serialize, Deserialize)]
pub struct CreateClaims {
    /// Database ID of the user
    pub sub: i64,
    /// Email of the user
    pub email: String,
    /// Whether the user has verified their email
    pub email_verified: Option<bool>,
    /// Name of the user
    pub name: Option<String>,
    /// First name of the user
    pub given_name: Option<String>,
    /// Last name of the user
    pub family_name: Option<String>,
    /// Locale of the user
    pub locale: Option<String>,
    /// Roles of the user granting access to parts of the platform
    pub roles: Vec<String>,
    /// Provider of the user
    pub provider: String,
    /// Time when the verification code was created
    pub email_code_created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub aud: Vec<String>,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    #[serde(flatten)]
    pub data: CreateClaims,
}

impl Claims {
    fn new(data: CreateClaims, issuer: &str, audience: &[String], expiration: i64) -> Self {
        let iat = chrono::Utc::now().to_utc().timestamp();

        Self {
            iss: issuer.to_string(),
            aud: audience.to_vec(),
            exp: iat + expiration,
            iat,
            nbf: iat,
            data,
        }
    }
}
