pub mod agent;
pub mod listener;
pub mod login;
pub mod server;
pub mod user;

use crate::models::state::AppState;
use common::error::Result;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    RsaPrivateKey,
};
use tauri::ipc::Invoke;

pub fn make_handlers() -> impl Fn(Invoke<tauri::Wry>) -> bool {
    tauri::generate_handler![
        is_setup_required,
        generate_keys,
        server::add_server,
        server::all_servers,
        server::remove_server,
        listener::add_listener,
        listener::all_listeners,
        listener::remove_listener,
        listener::start_listener,
        listener::stop_listener,
        agent::check_agent,
        agent::agents,
        login::login,
        user::user_create,
        quit
    ]
}

#[tauri::command]
pub async fn is_setup_required(state: tauri::State<'_, AppState>) -> Result<bool> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM users
        "#,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(count == 0)
}

#[tauri::command]
pub async fn generate_keys() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;

    Ok((
        priv_key.to_pkcs1_pem(LineEnding::CRLF)?.to_string(),
        priv_key
            .to_public_key()
            .to_pkcs1_pem(LineEnding::CRLF)?
            .to_string(),
    ))
}

#[tauri::command]
fn quit(handle: tauri::AppHandle) {
    handle.exit(0);
}
