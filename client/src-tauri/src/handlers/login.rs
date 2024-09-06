use crate::models::state::AppState;
use common::{
    crypt::{aes::decrypt, hash::verify_password_hash},
    db::{server::get_servers, user::get_user},
    error::Error,
    models::user::{Credentials, User},
};
use comms::event::ConnectionBuilder;

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
        let host = format!("{}:{}", server.server.host, server.server.event_port);
        let password = decrypt(&state.key.read().unwrap(), &server.server.password).unwrap();

        let connection = ConnectionBuilder::new(app_handle.clone())
            .auth(server.server.username, password)
            .server(host)
            .id(server.id)
            .build();

        state.eventlogs.connect(connection).await;
    }
}
