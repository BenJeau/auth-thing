use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use tracing::instrument;

#[instrument(skip_all)]
pub fn hash_password(password: &str) -> password_hash::Result<String> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
}

#[instrument(skip_all)]
pub fn verify_password(password: &str, hash: &str) -> password_hash::Result<bool> {
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(hash)?)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_password_when_hashed_then_it_can_be_verified() {
        let password = "a_very_safe_password";
        let hash = hash_password(password).unwrap();

        let result = verify_password(password, &hash).unwrap();

        assert!(result);
    }

    #[test]
    fn test_given_password_when_hashed_then_an_invalid_cannot_be_verified() {
        let password = "a_very_safe_password";
        let hash = hash_password(password).unwrap();

        let wrong_password = "a_not_very_safe_password";
        let result = verify_password(wrong_password, &hash).unwrap();

        assert!(!result);
    }
}
