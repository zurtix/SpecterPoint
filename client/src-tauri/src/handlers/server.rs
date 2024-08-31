use crate::models::state::AppState;
use common::crypt::aes::encrypt;
use common::db::server::{create_server, delete_server, get_servers};
use common::error::Result;
use common::models::server::{Server, ServerBase};
use eventlogs::event::ConnectionBuilder;

#[tauri::command]
pub async fn add_server(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    server: ServerBase,
) -> Result<()> {
    let encrypted_password = encrypt(&state.key.read().unwrap(), &server.password)?;

    let id = create_server(state.pool.clone(), server.clone(), encrypted_password).await?;
    let host = format!("{}:{}", server.host, server.event_port);
    let connection = ConnectionBuilder::new(app_handle.clone())
        .auth(server.username, server.password)
        .server(host)
        .id(id)
        .build();

    state.eventlogs.connect(connection).await;

    Ok(())
}

#[tauri::command]
pub async fn remove_server(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    delete_server(state.pool.clone(), &id).await?;
    state.eventlogs.disconnect(&id).await;
    Ok(())
}

#[tauri::command]
pub async fn all_servers(state: tauri::State<'_, AppState>) -> Result<Vec<Server>> {
    get_servers(state.pool.clone()).await
}
