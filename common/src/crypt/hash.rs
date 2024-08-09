use crate::error::Error;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn generate_password_hash(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password_hash(password_hash: String, password: &str) -> Result<bool, Error> {
    let hash = PasswordHash::new(&password_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}

#[cfg(test)]
mod hash_tests {
    use super::*;

    const DATA: &str = "specterpoint";

    #[test]
    fn successfully_generate_hash() {
        let hash = generate_password_hash(DATA).unwrap();
        assert_ne!(DATA, hash);
    }

    #[test]
    fn successfully_verify_password() {
        let password_hash = generate_password_hash(DATA).unwrap();
        assert!(!verify_password_hash(password_hash.clone(), "invalid-password").unwrap());
        assert!(verify_password_hash(password_hash, DATA).unwrap());
    }
}
