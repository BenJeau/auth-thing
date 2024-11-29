use chrono::NaiveDateTime;
use database::{models::jwt_configs::ModifyJwtConfig, SqlitePool};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

use crate::{crypto, Error, Result};

struct JwtConfig {
    validation: Validation,
    expiration: i64,
    issuer: String,
    audience: Vec<String>,
    algorithm: Algorithm,
    private_key: EncodingKey,
    public_key: DecodingKey,
}

pub async fn jwt_validation(
    pool: &SqlitePool,
    crypto: &crypto::Crypto,
    token: &str,
) -> Result<Claims> {
    let application_id = extract_application_id_from_token(token, pool).await?;

    let jwt_configs = database::logic::jwt_configs::get_jwt_configs(pool, application_id).await?;

    for config in jwt_configs {
        let jwt_config = from_jwt_config(config, &crypto);

        if let Ok(claims) = get_claims(token, &jwt_config).await {
            return Ok(claims);
        }
    }

    Err(Error::UnableToValidateToken)
}

async fn extract_application_id_from_token(token: &str, pool: &SqlitePool) -> Result<i64> {
    let header = jsonwebtoken::decode_header(token)?;

    let mut validation = Validation::new(header.alg);
    validation.insecure_disable_signature_validation();

    let claims: Claims = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(&[]), // dummy key since we're not validating
        &validation,
    )?
    .claims;
    let application_id = database::logic::applications::get_application_id(pool, &claims.iss)
        .await?
        .ok_or(Error::UnableToValidateToken)?;

    Ok(application_id)
}

fn from_jwt_config(
    config: database::models::jwt_configs::JwtConfig,
    crypto: &crypto::Crypto,
) -> JwtConfig {
    let algorithm = config.algorithm.parse().unwrap();

    let mut validation = Validation::new(algorithm);
    validation.set_audience(&config.audience);
    validation.set_issuer(&[&config.issuer]);
    validation.validate_exp = true;

    JwtConfig {
        validation,
        expiration: config.expiration,
        issuer: config.issuer.clone(),
        audience: config.audience.clone(),
        algorithm,
        private_key: get_encoding_key(&config.private_key, &crypto).unwrap(),
        public_key: DecodingKey::from_ec_der(&config.public_key),
    }
}

fn get_encoding_key(encrypted_key: &[u8], crypto: &crypto::Crypto) -> Result<EncodingKey> {
    let key = crypto.decrypt(encrypted_key)?;
    Ok(EncodingKey::from_ec_der(key.as_bytes()))
}

async fn get_claims(token: &str, config: &JwtConfig) -> Result<Claims> {
    jsonwebtoken::decode(token, &config.public_key, &config.validation)
        .map(|t| t.claims)
        .map_err(Into::into)
}

async fn generate_keys(
    application_id: i64,
    issuer: String,
    pool: &SqlitePool,
    jwt_config: &ModifyJwtConfig,
    crypto: &crypto::Crypto,
) -> Result<JwtConfig> {
    let document = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new())?;
    let pair = Ed25519KeyPair::from_pkcs8(document.as_ref()).unwrap();

    let private_key = EncodingKey::from_ed_der(document.as_ref());
    let public_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

    database::logic::jwt_configs::create_jwt_config(
        pool,
        application_id,
        jwt_config,
        &crypto.encrypt(&document.as_ref())?,
        &pair.public_key().as_ref(),
    )
    .await?;

    Ok(JwtConfig {
        validation: Validation::new(jwt_config.algorithm.parse().unwrap()),
        expiration: jwt_config.expiration,
        issuer,
        audience: jwt_config.audience.clone(),
        algorithm: jwt_config.algorithm.parse().unwrap(),
        private_key,
        public_key,
    })
}

pub fn generate_jwt(
    claims: CreateClaims,
    config: database::models::jwt_configs::JwtConfig,
    crypto: &crypto::Crypto,
) -> Result<String> {
    let jwt_config = from_jwt_config(config, crypto);

    let iat = chrono::Utc::now().to_utc().timestamp();

    let all_claims = Claims {
        iss: jwt_config.issuer.clone(),
        aud: jwt_config.audience.clone(),
        exp: iat + jwt_config.expiration as i64,
        iat,
        nbf: iat,
        data: claims,
    };

    jsonwebtoken::encode(
        &Header::new(jwt_config.algorithm),
        &serde_json::to_value(all_claims)?,
        &jwt_config.private_key,
    )
    .map_err(Into::into)
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
