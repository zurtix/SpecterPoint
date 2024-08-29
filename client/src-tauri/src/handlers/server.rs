use crate::models::state::AppState;
use common::crypt::aes::encrypt;
use common::db::server::{create_server, delete_server, get_servers};
use common::error::Result;
use common::models::server::{Server, ServerBase};
use common::models::user::BaseCredential;

#[tauri::command]
pub async fn add_server(
    app_hanle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    server: ServerBase,
) -> Result<()> {
    let encrypted_password = encrypt(&state.key.read().unwrap(), &server.password)?;

    let id = create_server(state.pool.clone(), server.clone(), encrypted_password).await?;

    let creds = BaseCredential {
        username: server.username,
        password: server.password,
    };

    state
        .manager
        .add_connection(
            creds,
            id,
            format!("{}:{}", server.host, server.log_port)
                .parse()
                .unwrap(),
            app_hanle,
        )
        .await;

    Ok(())
}

#[tauri::command]
pub async fn remove_server(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    delete_server(state.pool.clone(), &id).await?;
    state.manager.remove_connection(id).await;
    Ok(())
}

#[tauri::command]
pub async fn all_servers(state: tauri::State<'_, AppState>) -> Result<Vec<Server>> {
    get_servers(state.pool.clone()).await
}
