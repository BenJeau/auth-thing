use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    let content = std::env::args().nth(1).unwrap();
    let hash = hash_password(&content).unwrap();
    println!("{hash}");
}

fn hash_password(password: &str) -> password_hash::Result<String> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
}
