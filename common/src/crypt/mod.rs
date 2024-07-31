use crate::error::Error;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit},
    Aes256Gcm, Key,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hex::{decode, encode};

pub fn generate_password_hash(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password_hash(password_hash: &str, password: &str) -> Result<bool, Error> {
    let hash = PasswordHash::new(password_hash)
        .map_err(|_| Error::Backend("Failed to load password hash".into()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}

pub fn encrypt(key: &str, data: &str) -> Result<String, Error> {
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
    let cipher = Aes256Gcm::new(k);
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(|_| Error::Backend("Failed to encrypt data".into()))?;
    Ok(encode([nonce.to_vec(), ciphertext].concat()))
}

pub fn decrypt(key: &str, encodedtext: &str) -> Result<String, Error> {
    let ciphertext =
        decode(encodedtext).map_err(|_| Error::Backend("Failed to decode data".into()))?;
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = GenericArray::from_slice(&ciphertext[..12]);
    let cipher = Aes256Gcm::new(k);
    let text = cipher
        .decrypt(nonce, &ciphertext[12..])
        .map_err(|_| Error::Backend("Failed to decrypt data".into()))?;

    Ok(String::from_utf8_lossy(text.as_slice()).to_string())
}

#[cfg(test)]
mod crypt_tests {
    use super::*;

    const KEY: &str = "12345678123456781234567812345678";
    const DATA: &str = "specterpoint";

    #[test]
    fn successfully_generate_hash() {
        let hash = generate_password_hash(DATA).unwrap();
        assert_ne!(DATA, hash);
    }

    #[test]
    fn successfully_verify_password() {
        let password_hash = generate_password_hash(DATA).unwrap();
        assert!(!verify_password_hash(&password_hash, "invalid-password").unwrap());
        assert!(verify_password_hash(&password_hash, DATA).unwrap());
    }

    #[test]
    fn successfully_encrypt_data() {
        let ciphertext = encrypt(KEY, DATA).unwrap();
        assert_ne!(DATA, ciphertext);
    }

    #[test]
    fn successfully_decrypt_data() {
        let encodedtext = encrypt(KEY, DATA).unwrap();
        let text = decrypt(KEY, &encodedtext).unwrap();
        assert_ne!(DATA, encodedtext);
        assert_eq!(DATA, text);
    }
}
