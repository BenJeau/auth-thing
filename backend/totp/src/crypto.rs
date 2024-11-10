use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fmt;

use crate::{key::generate_random_alphanumeric_string, Error, Result};

const DIGITS_POWER: [u32; 9] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
];

#[derive(Debug, Clone, Copy, Default)]
pub enum CryptoAlgorithm {
    #[default]
    Sha1,
    Sha256,
    Sha512,
}

impl fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoAlgorithm::Sha1 => write!(f, "SHA1"),
            CryptoAlgorithm::Sha256 => write!(f, "SHA256"),
            CryptoAlgorithm::Sha512 => write!(f, "SHA512"),
        }
    }
}

impl CryptoAlgorithm {
    fn compute_hmac(&self, key: &str, data: u64) -> Vec<u8> {
        match self {
            Self::Sha1 => {
                let mut mac = Hmac::<Sha1>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");
                mac.update(&data.to_be_bytes());
                mac.finalize().into_bytes().to_vec()
            }
            Self::Sha256 => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");
                mac.update(&data.to_be_bytes());
                mac.finalize().into_bytes().to_vec()
            }
            Self::Sha512 => {
                let mut mac = Hmac::<Sha512>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");
                mac.update(&data.to_be_bytes());
                mac.finalize().into_bytes().to_vec()
            }
        }
    }

    fn key_length(&self) -> usize {
        match self {
            Self::Sha1 => 20,
            Self::Sha256 => 32,
            Self::Sha512 => 64,
        }
    }

    pub fn generate_key(&self) -> String {
        generate_random_alphanumeric_string(self.key_length())
    }

    pub fn generate_totp(&self, key: &str, steps: u64, digits: u8) -> Result<String> {
        if digits as usize >= DIGITS_POWER.len() {
            return Err(Error::InvalidDigits(digits));
        }

        let hash = self.compute_hmac(key, steps);

        // Gets the last byte of the hash, then gets the last 4 bits of it
        let offset = (hash[hash.len() - 1] & 0xf) as usize;

        // Gets 4 bit of the hash starting at the offset
        let binary = u32::from_be_bytes([
            hash[offset] & 0x7f,
            hash[offset + 1],
            hash[offset + 2],
            hash[offset + 3],
        ]);

        // Gets the last digits of the subset of the hash (defined by _digits_)
        let otp = binary % DIGITS_POWER[digits as usize];

        Ok(format!("{:0>width$}", otp, width = digits as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_sha1_algorithm_then_key_length_is_40() {
        let key = CryptoAlgorithm::Sha1.generate_key();

        assert_eq!(key.len(), 20);
    }

    #[test]
    fn test_given_sha256_algorithm_then_key_length_is_64() {
        let key = CryptoAlgorithm::Sha256.generate_key();

        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_given_sha512_algorithm_then_key_length_is_128() {
        let key = CryptoAlgorithm::Sha512.generate_key();

        assert_eq!(key.len(), 64);
    }

    #[test]
    fn test_given_digits_is_greater_than_digits_power_then_error_is_returned() {
        let result = CryptoAlgorithm::Sha256.generate_totp("1234567890123456", 0, 10);

        assert_eq!(result, Err(Error::InvalidDigits(10)));
    }

    #[test]
    fn test_given_key_and_text_when_hashing_with_sha1_then_is_correct() {
        let key = "12345678901234567890";
        let data = 1234567890u64;

        let hash = CryptoAlgorithm::Sha1.compute_hmac(key, data);

        assert_eq!(
            hash,
            vec![
                74, 12, 104, 52, 83, 65, 171, 209, 162, 207, 222, 20, 111, 223, 154, 146, 22, 33,
                165, 125
            ]
        );
    }

    #[test]
    fn test_given_key_and_text_when_hashing_with_sha256_then_is_correct() {
        let key = "12345678901234567890123456789012";
        let data = 1234567890;

        let hash = CryptoAlgorithm::Sha256.compute_hmac(key, data);

        assert_eq!(
            hash,
            vec![
                20, 236, 42, 55, 140, 162, 239, 182, 72, 235, 249, 224, 175, 129, 42, 16, 197, 5,
                120, 99, 64, 242, 194, 130, 106, 82, 45, 247, 58, 33, 38, 142
            ]
        );
    }

    #[test]
    fn test_given_key_and_text_when_hashing_with_sha512_then_is_correct() {
        let key = "1234567890123456789012345678901234567890123456789012345678901234";
        let data = 1234567890;

        let hash = CryptoAlgorithm::Sha512.compute_hmac(key, data);

        assert_eq!(
            hash,
            vec![
                143, 123, 15, 87, 40, 208, 5, 94, 125, 227, 64, 238, 233, 120, 188, 75, 69, 33,
                121, 47, 165, 35, 53, 131, 224, 78, 238, 107, 195, 1, 238, 89, 75, 170, 155, 200,
                242, 106, 224, 94, 96, 58, 126, 234, 70, 166, 239, 89, 252, 15, 169, 64, 13, 221,
                58, 206, 55, 218, 174, 21, 120, 72, 18, 33
            ]
        );
    }
}
