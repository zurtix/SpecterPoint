use base64::prelude::*;
use common::crypt::aes;
use rand::{distributions::Alphanumeric, Rng};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};

const PUBLIC_RSA_KEY: &str = r#"
PLACE LISTENER PUB RSA KEY HERE
"#;

fn encrypt(message: &str) -> String {
    let mut rng = rand::thread_rng();

    let key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let aes_ciphertext = aes::encrypt(&key, message).expect("Failed to encrypt");
    let payload = format!("{}{}", key, aes_ciphertext);
    let pubk = RsaPublicKey::from_pkcs1_pem(PUBLIC_RSA_KEY).expect("Failed to read public key");
    let rsa_ciphertext = pubk
        .encrypt(&mut rng, Pkcs1v15Encrypt, payload.as_bytes())
        .expect("RSA encrypt failed");

    BASE64_STANDARD.encode(rsa_ciphertext).to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fake_bearer = encrypt(r#"{"agent": {"id": "for testing"}}"#);
    let client = reqwest::Client::new();

    let res = client
        .get("http://127.0.0.1:9999/index.html")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", fake_bearer),
        )
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        println!("Sent!");
    } else {
        println!("{:?}", res)
    }

    Ok(())
}
