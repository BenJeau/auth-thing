use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    claims::{decode_jwt, encode_jwt},
    keys::{
        generate_ed25519_keys, generate_esdca_keys, generate_hmac_keys, generate_rsa_keys, RawKeys,
        ES256_SIGNING_ALG, ES384_SIGNING_ALG,
    },
    Error, Result,
};

#[derive(Copy, Clone)]
pub enum Algorithm {
    EdDSA,
    ES256,
    ES384,
    RS256,
    RS384,
    RS512,
    HS256,
    HS384,
    HS512,
}

impl Algorithm {
    pub fn jwt_encoding_key(&self, key: &[u8]) -> EncodingKey {
        match self {
            Algorithm::EdDSA => EncodingKey::from_ec_der(key),
            Algorithm::ES256 | Algorithm::ES384 => EncodingKey::from_ed_der(key),
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                EncodingKey::from_rsa_der(key)
            }
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => EncodingKey::from_secret(key),
        }
    }

    pub fn jwt_decoding_key(&self, key: &[u8]) -> DecodingKey {
        match self {
            Algorithm::EdDSA => DecodingKey::from_ec_der(key),
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ed_der(key),
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                DecodingKey::from_rsa_der(key)
            }
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => DecodingKey::from_secret(key),
        }
    }

    pub fn generate_keys(&self) -> Result<RawKeys> {
        match self {
            Algorithm::EdDSA => generate_ed25519_keys(),
            Algorithm::ES256 => generate_esdca_keys(ES256_SIGNING_ALG),
            Algorithm::ES384 => generate_esdca_keys(ES384_SIGNING_ALG),
            Algorithm::RS256 => generate_rsa_keys(2048),
            Algorithm::RS384 => generate_rsa_keys(3072),
            Algorithm::RS512 => generate_rsa_keys(4096),
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => generate_hmac_keys(),
        }
    }

    pub fn encode_jwt<T: Serialize>(&self, key: &[u8], claims: &T) -> Result<String> {
        encode_jwt(&self.jwt_encoding_key(key), *self, claims)
    }

    pub fn decode_jwt<T: DeserializeOwned>(
        &self,
        key: &[u8],
        issuer: &[String],
        audience: &[String],
        token: &str,
    ) -> Result<T> {
        decode_jwt(&self.jwt_decoding_key(key), *self, issuer, audience, token)
    }
}

impl From<Algorithm> for jsonwebtoken::Algorithm {
    fn from(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::EdDSA => jsonwebtoken::Algorithm::EdDSA,
            Algorithm::ES256 => jsonwebtoken::Algorithm::ES256,
            Algorithm::ES384 => jsonwebtoken::Algorithm::ES384,
            Algorithm::RS256 => jsonwebtoken::Algorithm::RS256,
            Algorithm::RS384 => jsonwebtoken::Algorithm::RS384,
            Algorithm::RS512 => jsonwebtoken::Algorithm::RS512,
            Algorithm::HS256 => jsonwebtoken::Algorithm::HS256,
            Algorithm::HS384 => jsonwebtoken::Algorithm::HS384,
            Algorithm::HS512 => jsonwebtoken::Algorithm::HS512,
        }
    }
}

impl TryFrom<String> for Algorithm {
    type Error = Error;

    fn try_from(algorithm: String) -> Result<Self> {
        match algorithm.as_str() {
            "EdDSA" => Ok(Algorithm::EdDSA),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            _ => Err(Error::InvalidAlgorithm(algorithm)),
        }
    }
}
