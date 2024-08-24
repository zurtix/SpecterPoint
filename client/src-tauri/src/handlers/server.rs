use crate::models::state::AppState;
use common::crypt::aes::encrypt;
use common::db::server::{create_server, get_servers};
use common::error::Error;
use common::models::server::{Server, ServerBase};

#[tauri::command]
pub async fn add_server(
    state: tauri::State<'_, AppState>,
    server: ServerBase,
) -> Result<(), Error> {
    let encrypted_password = encrypt(&state.key.read().unwrap(), &server.password)?;
    Ok(create_server(state.pool.clone(), server, encrypted_password).await?)
}

#[tauri::command]
pub async fn all_servers(state: tauri::State<'_, AppState>) -> Result<Vec<Server>, Error> {
    Ok(get_servers(state.pool.clone()).await?)
}
