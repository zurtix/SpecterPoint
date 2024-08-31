use std::{net::SocketAddr, str::FromStr};

use crate::models::state::AppState;
use common::{
    crypt::{aes::decrypt, hash::verify_password_hash},
    db::{server::get_servers, user::get_user},
    error::Error,
    models::user::{BaseCredential, Credentials, User},
};

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    creds: Credentials,
) -> Result<(), Error> {
    let user: User = get_user(state.pool.clone(), &creds.auth.username).await?;

    if verify_password_hash(user.password, &creds.auth.password)? {
        if let Some(key) = creds.key {
            state.set_key(key.to_string());
        }

        connect_logs(app_handle, state).await;

        Ok(())
    } else {
        Err(Error::Auth)
    }
}

async fn connect_logs(app_handle: tauri::AppHandle, state: tauri::State<'_, AppState>) {
    let servers = get_servers(state.pool.clone()).await.unwrap();
    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password).unwrap();
        state
            .manager
            .add_connection(
                BaseCredential {
                    username: server.server.username,
                    password,
                },
                server.id,
                SocketAddr::from_str(&format!(
                    "{}:{}",
                    server.server.host, server.server.event_port
                ))
                .unwrap(),
                app_handle.clone(),
            )
            .await;
    }
}
