use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead},
    AeadCore, KeyInit, XChaCha20Poly1305,
};
use rand::{rngs::OsRng, thread_rng, Rng};
use tracing::instrument;

use crate::{Error, Result};

#[derive(Clone)]
pub struct Crypto {
    chacha_cipher: XChaCha20Poly1305,
}

impl Crypto {
    #[instrument(skip_all)]
    pub fn new<K: Into<String>>(key: K) -> Result<Self> {
        Ok(Self {
            chacha_cipher: XChaCha20Poly1305::new_from_slice(key.into().as_bytes())
                .map_err(|_| Error::ChaChaSecretLength)?,
        })
    }

    #[instrument(skip_all, err)]
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = self.chacha_cipher.encrypt(&nonce, data)?;

        let mut encrypted_data = Vec::with_capacity(nonce.len() + ciphertext.len());
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&ciphertext);

        Ok(encrypted_data)
    }

    #[instrument(skip_all, err)]
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let (raw_nonce, ciphertext) = data.split_at(24);
        let nonce = GenericArray::from_slice(raw_nonce);
        let decrypted_data = self.chacha_cipher.decrypt(nonce, ciphertext)?;

        Ok(decrypted_data)
    }

    #[instrument(skip_all)]
    pub fn generate_random_numeric_string(&self, length: usize) -> String {
        let mut rng = thread_rng();

        std::iter::repeat(())
            .map(|()| rng.gen_range(0..=9))
            .map(char::from)
            .take(length)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_data_encrypting_data_can_be_decrypted() {
        let crypto = Crypto::new("oXcQAC8U@KNY8MvH@WhRV4vEMGpMbAvi").unwrap();

        let data = "super_secret_data_to_save";

        let encrypted_data = crypto.encrypt(data.as_bytes()).unwrap();
        let decrypted_data = crypto.decrypt(&encrypted_data).unwrap();

        assert_eq!(decrypted_data, data.as_bytes());
    }
}
