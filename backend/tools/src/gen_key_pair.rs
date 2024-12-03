use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, KeyInit, XChaCha20Poly1305,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use ring::{
    rand::SecureRandom,
    signature::{self, KeyPair},
};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

fn main() {
    let secret_key = std::env::args().nth(1).unwrap();

    let algorithms = vec![
        Algorithm::HS256,
        Algorithm::HS384,
        Algorithm::HS512,
        Algorithm::EdDSA,
        Algorithm::ES256,
        Algorithm::ES384,
        Algorithm::RS256,
        Algorithm::RS384,
        Algorithm::RS512,
    ];

    let claims = Claims {
        sub: "test".to_string(),
        exp: 120000000000,
    };

    let rng = ring::rand::SystemRandom::new();

    for algorithm in algorithms {
        encode_and_decode_jwt(&rng, algorithm, &claims, secret_key.as_bytes());
    }
}

fn encode_and_decode_jwt(
    rng: &ring::rand::SystemRandom,
    algorithm: Algorithm,
    claims: &Claims,
    secret_key: &[u8],
) {
    let (encoding_key, decoding_key) = get_keys_from_algorithm(rng, algorithm, secret_key).unwrap();

    let token_header = jsonwebtoken::Header::new(algorithm);
    let token = encode(&token_header, claims, &encoding_key).unwrap();
    decode::<Claims>(&token, &decoding_key, &Validation::new(algorithm)).unwrap();

    println!("Algorithm: {:?}", algorithm);
    println!("Token: {}\n", token);
}

fn get_keys_from_algorithm(
    rng: &ring::rand::SystemRandom,
    algorithm: Algorithm,
    secret_key: &[u8],
) -> Result<(EncodingKey, DecodingKey), Box<dyn std::error::Error>> {
    match algorithm {
        Algorithm::ES256 | Algorithm::ES384 => {
            let signing = match algorithm {
                Algorithm::ES256 => &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
                Algorithm::ES384 => &signature::ECDSA_P384_SHA384_ASN1_SIGNING,
                _ => unreachable!(),
            };

            let document = signature::EcdsaKeyPair::generate_pkcs8(signing, rng)?;
            let key_pair = signature::EcdsaKeyPair::from_pkcs8(signing, document.as_ref(), rng)?;
            print_data(
                document.as_ref(),
                key_pair.public_key().as_ref(),
                secret_key,
            );

            let encoding_key = EncodingKey::from_ec_der(document.as_ref());
            let decoding_key = DecodingKey::from_ec_der(key_pair.public_key().as_ref());
            Ok((encoding_key, decoding_key))
        }
        Algorithm::EdDSA => {
            let document = signature::Ed25519KeyPair::generate_pkcs8(rng)?;
            let key_pair = signature::Ed25519KeyPair::from_pkcs8(document.as_ref())?;
            print_data(
                document.as_ref(),
                key_pair.public_key().as_ref(),
                secret_key,
            );
            let encoding_key = EncodingKey::from_ed_der(document.as_ref());
            let decoding_key = DecodingKey::from_ed_der(key_pair.public_key().as_ref());
            Ok((encoding_key, decoding_key))
        }
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            let mut secret = vec![0u8; 32];
            rng.fill(&mut secret)?;
            print_data(&secret, &secret, secret_key);
            let encoding_key = EncodingKey::from_secret(&secret);
            let decoding_key = DecodingKey::from_secret(&secret);
            Ok((encoding_key, decoding_key))
        }
        Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
            let mut rng = rand::thread_rng();
            let bits = match algorithm {
                Algorithm::RS256 => 2048,
                Algorithm::RS384 => 3072,
                Algorithm::RS512 => 4096,
                _ => unreachable!(),
            };
            let priv_key =
                rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            let pub_key = rsa::RsaPublicKey::from(&priv_key);

            print_data(
                priv_key.to_pkcs1_der().unwrap().as_bytes(),
                pub_key.to_pkcs1_der().unwrap().as_bytes(),
                secret_key,
            );

            priv_key
                .write_pkcs1_der_file("priv.der")
                .expect("failed to write private key");
            pub_key
                .write_pkcs1_der_file("pub.der")
                .expect("failed to write public key");

            let encoding_key =
                EncodingKey::from_rsa_der(priv_key.to_pkcs1_der().unwrap().as_bytes());
            let decoding_key =
                DecodingKey::from_rsa_der(pub_key.to_pkcs1_der().unwrap().as_bytes());
            Ok((encoding_key, decoding_key))
        }
        _ => unimplemented!(),
    }
}

pub fn encrypt(data: &[u8], secret_key: &[u8]) -> Vec<u8> {
    let chacha_cipher = XChaCha20Poly1305::new_from_slice(secret_key).unwrap();
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = chacha_cipher.encrypt(&nonce, data).unwrap();

    let mut encrypted_data = Vec::with_capacity(nonce.len() + ciphertext.len());
    encrypted_data.extend_from_slice(&nonce);
    encrypted_data.extend_from_slice(&ciphertext);

    encrypted_data
}

fn print_data(private_key: &[u8], public_key: &[u8], secret_key: &[u8]) {
    println!("Unencrypted private key: {}", hex::encode(private_key));
    println!(
        "Private key: {}",
        hex::encode(encrypt(private_key, secret_key))
    );
    println!("Unencrypted public key: {}", hex::encode(public_key));
    println!(
        "Public key: {}",
        hex::encode(encrypt(public_key, secret_key))
    );
}
