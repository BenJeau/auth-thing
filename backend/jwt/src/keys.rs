use ring::{
    rand::SecureRandom,
    signature::{
        EcdsaKeyPair, EcdsaSigningAlgorithm, Ed25519KeyPair, KeyPair,
        ECDSA_P256_SHA256_ASN1_SIGNING, ECDSA_P384_SHA384_ASN1_SIGNING,
    },
};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use std::sync::LazyLock;

use crate::Result;

const HMAC_KEY_SIZE: usize = 32;
static RNG: LazyLock<ring::rand::SystemRandom> = LazyLock::new(ring::rand::SystemRandom::new);

pub static ES256_SIGNING_ALG: &EcdsaSigningAlgorithm = &ECDSA_P256_SHA256_ASN1_SIGNING;
pub static ES384_SIGNING_ALG: &EcdsaSigningAlgorithm = &ECDSA_P384_SHA384_ASN1_SIGNING;

pub struct RawKey {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub(crate) fn generate_rsa_keys(bits: usize) -> Result<RawKey> {
    let priv_key = rsa::RsaPrivateKey::new(&mut rand::thread_rng(), bits)?;
    let pub_key = rsa::RsaPublicKey::from(&priv_key);

    Ok(RawKey {
        private_key: priv_key.to_pkcs1_der()?.as_bytes().to_vec(),
        public_key: pub_key.to_pkcs1_der()?.as_bytes().to_vec(),
    })
}

pub(crate) fn generate_hmac_keys() -> Result<RawKey> {
    let mut secret_key = vec![0u8; HMAC_KEY_SIZE];
    RNG.fill(&mut secret_key)?;

    Ok(RawKey {
        private_key: secret_key.clone(),
        public_key: secret_key,
    })
}

pub(crate) fn generate_ed25519_keys() -> Result<RawKey> {
    let document = Ed25519KeyPair::generate_pkcs8(&RNG.to_owned())?;
    let key_pair = Ed25519KeyPair::from_pkcs8(document.as_ref())?;

    Ok(RawKey {
        private_key: document.as_ref().to_vec(),
        public_key: key_pair.public_key().as_ref().to_vec(),
    })
}

pub(crate) fn generate_esdca_keys(signing_alg: &'static EcdsaSigningAlgorithm) -> Result<RawKey> {
    let document = EcdsaKeyPair::generate_pkcs8(signing_alg, &RNG.to_owned())?;
    let key_pair = EcdsaKeyPair::from_pkcs8(signing_alg, document.as_ref(), &RNG.to_owned())?;

    Ok(RawKey {
        private_key: document.as_ref().to_vec(),
        public_key: key_pair.public_key().as_ref().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use rsa::{
        pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
        traits::PublicKeyParts,
    };

    use super::*;

    #[test]
    fn test_given_number_of_bits_when_generating_rsa_keys_then_valid_rsa_keys_are_generated_with_correct_length(
    ) {
        let number_of_bits = 2048;
        let number_of_bytes = number_of_bits / 8;

        let key = generate_rsa_keys(number_of_bits).unwrap();

        let private_key = rsa::RsaPrivateKey::from_pkcs1_der(&key.private_key).unwrap();
        let public_key = rsa::RsaPublicKey::from_pkcs1_der(&key.public_key).unwrap();

        assert_eq!(private_key.size(), number_of_bytes);
        assert_eq!(public_key.size(), number_of_bytes);
        assert_eq!(private_key.to_public_key(), public_key);
    }

    #[test]
    fn test_when_generating_hmac_keys_then_private_and_public_keys_are_the_same() {
        let key = generate_hmac_keys().unwrap();

        assert_eq!(key.private_key, key.public_key);
    }

    #[test]
    fn test_when_generating_hmac_keys_then_valid_hmac_keys_are_generated() {
        let key = generate_hmac_keys().unwrap();

        assert_eq!(key.private_key.len(), HMAC_KEY_SIZE);
        assert_eq!(key.public_key.len(), HMAC_KEY_SIZE);
    }

    #[test]
    fn test_when_generating_ed25519_keys_then_valid_ed25519_keys_are_generated() {
        let key = generate_ed25519_keys().unwrap();

        let private_key = Ed25519KeyPair::from_pkcs8(&key.private_key).unwrap();
        let public_key = private_key.public_key();

        assert_eq!(private_key.public_key().as_ref().len(), 32);
        assert_eq!(public_key.as_ref(), &key.public_key);
    }

    #[test]
    fn test_when_generating_esdca_keys_then_able_to_reconstruct_public_key_from_private_key() {
        let key = generate_esdca_keys(ES256_SIGNING_ALG).unwrap();

        let private_key =
            EcdsaKeyPair::from_pkcs8(ES256_SIGNING_ALG, &key.private_key, &RNG.to_owned()).unwrap();
        let public_key = private_key.public_key();

        assert_eq!(public_key.as_ref(), &key.public_key);
    }
}
