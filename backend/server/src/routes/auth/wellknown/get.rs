use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Serialize;
use sha1::{Digest, Sha1};
use utoipa::ToSchema;

use crate::{Result, ServerState};

#[derive(Serialize, ToSchema)]
struct Jwks {
    keys: Vec<JwkKey>,
}

#[derive(Serialize, ToSchema, Default)]
struct JwkKey {
    kty: String,
    #[serde(rename = "use")]
    key_use: String,
    kid: String,
    alg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    crv: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    x: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    y: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    n: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    e: String,
}

/// Get JSON Web Key Set for an application
#[utoipa::path(
    get,
    path = "",
    params(
        ("application_slug" = String, Path, description = "Application slug")
    ),
    responses(
        (status = 200, description = "JSON Web Key Set", body = Jwks),
        (status = 404, description = "Application not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn get_jwks(
    State(state): State<ServerState>,
    Path(application_slug): Path<String>,
) -> Result<impl IntoResponse> {
    let application_id =
        database::logic::applications::get_application_id(&state.pool, &application_slug)
            .await?
            .ok_or_else(|| crate::error::Error::NotFound("Application not found".into()))?;

    let jwt_configs =
        database::logic::jwt_configs::get_jwt_configs(&state.pool, application_id).await?;

    let keys = jwt_configs
        .into_iter()
        .flat_map(|config| {
            let public_key = state.crypto.decrypt(&config.public_key).unwrap();

            let kid = {
                let mut hasher = Sha1::new();
                hasher.update(&public_key);
                STANDARD.encode(hex::encode(hasher.finalize()))
            };

            let encoded_public_key = STANDARD.encode(&public_key);

            match config.algorithm.as_str() {
                "RS256" => Some(JwkKey {
                    kty: "RSA".to_string(),
                    key_use: "sig".to_string(),
                    kid,
                    alg: "RS256".to_string(),
                    n: encoded_public_key.clone(),
                    e: "AQAB".to_string(),
                    ..Default::default()
                }),
                "RS384" => Some(JwkKey {
                    kty: "RSA".to_string(),
                    key_use: "sig".to_string(),
                    kid,
                    alg: "RS384".to_string(),
                    n: encoded_public_key.clone(),
                    e: "AQAB".to_string(),
                    ..Default::default()
                }),
                "RS512" => Some(JwkKey {
                    kty: "RSA".to_string(),
                    key_use: "sig".to_string(),
                    kid,
                    alg: "RS512".to_string(),
                    n: encoded_public_key.clone(),
                    e: "AQAB".to_string(),
                    ..Default::default()
                }),
                "ES256" => {
                    let private_key = state.crypto.decrypt(&config.private_key).unwrap();
                    let (x, y) = jwt::get_es256_coordinates(&private_key);

                    Some(JwkKey {
                        kty: "EC".to_string(),
                        key_use: "sig".to_string(),
                        kid,
                        alg: "ES256".to_string(),
                        crv: "P-256".to_string(),
                        x: STANDARD.encode(x),
                        y: STANDARD.encode(y),
                        ..Default::default()
                    })
                }
                "ES384" => {
                    let private_key = state.crypto.decrypt(&config.private_key).unwrap();
                    let (x, y) = jwt::get_es384_coordinates(&private_key);

                    Some(JwkKey {
                        kty: "EC".to_string(),
                        key_use: "sig".to_string(),
                        kid,
                        alg: "ES384".to_string(),
                        crv: "P-384".to_string(),
                        x: STANDARD.encode(x),
                        y: STANDARD.encode(y),
                        ..Default::default()
                    })
                }
                "EdDSA" => Some(JwkKey {
                    kty: "OKP".to_string(),
                    key_use: "sig".to_string(),
                    kid,
                    alg: "EdDSA".to_string(),
                    crv: "Ed25519".to_string(),
                    x: encoded_public_key,
                    ..Default::default()
                }),
                _ => None,
            }
        })
        .collect();

    Ok(Json(Jwks { keys }))
}
