use crate::models::state::AppState;
use common::crypt::aes::encrypt;
use common::db::server::{create_server, get_servers};
use common::error::Error;
use common::models::server::{Server, ServerBase};
use common::models::user::Credentials;

#[tauri::command]
pub async fn add_server(
    app_hanle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    server: ServerBase,
) -> Result<(), Error> {
    let encrypted_password = encrypt(&state.key.read().unwrap(), &server.password)?;

    let id = create_server(state.pool.clone(), server.clone(), encrypted_password).await?;
    let creds = Credentials {
        username: server.username,
        password: server.password,
        key: None,
    };

    println!("Adding to TCP Manager");
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
pub async fn all_servers(state: tauri::State<'_, AppState>) -> Result<Vec<Server>, Error> {
    Ok(get_servers(state.pool.clone()).await?)
}
