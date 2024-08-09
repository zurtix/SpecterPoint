use crate::error::Error;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit},
    Aes256Gcm, Key,
};
use hex::{decode, encode};

pub fn encrypt(key: &str, data: &str) -> Result<String, Error> {
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
    let cipher = Aes256Gcm::new(k);
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;

    Ok(encode([nonce.to_vec(), ciphertext].concat()))
}

pub fn decrypt(key: &str, encodedtext: &str) -> Result<String, Error> {
    let ciphertext = decode(encodedtext)?;
    let k = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = GenericArray::from_slice(&ciphertext[..12]);
    let cipher = Aes256Gcm::new(k);
    let text = cipher.decrypt(nonce, &ciphertext[12..])?;

    Ok(String::from_utf8_lossy(text.as_slice()).to_string())
}

#[cfg(test)]
mod crypt_tests {
    use super::*;

    const KEY: &str = "12345678123456781234567812345678";
    const DATA: &str = "specterpoint";

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
