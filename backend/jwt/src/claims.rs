use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, TokenData, Validation};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Algorithm, Result};

pub(crate) fn encode_jwt<T: Serialize>(
    encoding_key: &EncodingKey,
    algorithm: Algorithm,
    claims: &T,
) -> Result<String> {
    let token_header = jsonwebtoken::Header::new(algorithm.into());
    let token = encode(&token_header, claims, &encoding_key)?;

    Ok(token)
}

pub(crate) fn decode_jwt<T: DeserializeOwned>(
    decoding_key: &DecodingKey,
    algorithm: Algorithm,
    issuer: &[&str],
    audience: &[&str],
    token: &str,
) -> Result<T> {
    let mut validation = Validation::new(algorithm.into());
    validation.set_audience(audience);
    validation.set_issuer(issuer);
    let token_data = decode::<T>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::sync::LazyLock;

    const EXPIRED_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzdWIiLCJleHAiOjEwMDAsImlzcyI6ImlzcyIsImF1ZCI6ImF1ZCJ9.jhW4kyIaUOUobQLUTPINTdwsKyF3rkTWVZE0afEmwKQ";
    const VALID_HS256_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzdWIiLCJleHAiOjEwMDAwMDAwMDAwLCJpc3MiOiJpc3MiLCJhdWQiOiJhdWQifQ.Mi4t_Crm8iceXIGr7vPgMqYjSuKSJ1feAiYxaxJ1KxE";
    const ENCODING_KEY: LazyLock<EncodingKey> =
        LazyLock::new(|| EncodingKey::from_secret(b"secret"));
    const DECODING_KEY: LazyLock<DecodingKey> =
        LazyLock::new(|| DecodingKey::from_secret(b"secret"));

    #[derive(Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: i64,
        iss: String,
        aud: String,
    }

    impl Claims {
        fn new() -> Self {
            Self {
                sub: "sub".to_string(),
                exp: 10000000000,
                iss: "iss".to_string(),
                aud: "aud".to_string(),
            }
        }
    }

    #[test]
    fn test_when_encoding_jwt_then_produces_string_jwt() {
        let token = encode_jwt(&ENCODING_KEY, Algorithm::HS256, &Claims::new()).unwrap();

        assert_eq!(token, VALID_HS256_TOKEN);
    }

    #[test]
    fn test_given_invalid_algorithm_with_encoding_key_when_encoding_jwt_then_error_is_returned() {
        let result = encode_jwt(&ENCODING_KEY, Algorithm::ES256, &Claims::new());

        assert!(result.is_err());
    }

    #[test]
    fn test_given_valid_issuer_and_audience_when_decoding_jwt_then_token_is_valid() {
        let token_data = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::HS256,
            &["iss"],
            &["aud"],
            &VALID_HS256_TOKEN,
        )
        .unwrap();

        assert_eq!(token_data.sub, "sub");
        assert_eq!(token_data.exp, 10000000000);
        assert_eq!(token_data.iss, "iss");
        assert_eq!(token_data.aud, "aud");
    }

    #[test]
    fn test_given_expired_token_when_decoding_jwt_then_error_is_returned() {
        let result = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::HS256,
            &["iss"],
            &["aud"],
            &EXPIRED_TOKEN,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_given_invalid_audience_when_decoding_jwt_then_error_is_returned() {
        let result = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::HS256,
            &["iss"],
            &["invalid_audience"],
            &VALID_HS256_TOKEN,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_given_invalid_issuer_when_decoding_jwt_then_error_is_returned() {
        let result = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::HS256,
            &["invalid_issuer"],
            &["aud"],
            &VALID_HS256_TOKEN,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_given_invalid_algorithm_with_decoding_key_when_decoding_jwt_then_error_is_returned() {
        let result = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::ES256,
            &["iss"],
            &["aud"],
            &VALID_HS256_TOKEN,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_given_invalid_token_when_decoding_jwt_then_error_is_returned() {
        let result = decode_jwt::<Claims>(
            &DECODING_KEY,
            Algorithm::HS256,
            &["iss"],
            &["aud"],
            &"token",
        );

        assert!(result.is_err());
    }
}
