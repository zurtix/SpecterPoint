use crate::{httpclient::ClientBuilder, models::state::AppState};
use common::{
    crypt::aes::decrypt,
    db::{
        listener::{create_listener, delete_listener, get_listener, get_listseners},
        server::{create_server_listeners, get_server, get_server_ids, get_servers},
    },
    error::Result,
    models::listener::{ListenerBaseWithEndpoints, ListenerWithEndpoints},
};
use serde_json::Value;

#[tauri::command]
pub async fn add_listener(
    state: tauri::State<'_, AppState>,
    create: ListenerBaseWithEndpoints,
) -> Result<()> {
    let listener_id = create_listener(state.pool.clone(), &create).await?;
    let server_ids = get_server_ids(state.pool.clone()).await?;
    let listener = get_listener(state.pool.clone(), &listener_id).await?;

    for server_id in server_ids {
        let server = get_server(state.pool.clone(), &server_id).await?;

        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;

        let client = ClientBuilder::new(&server.server)
            .auth(server.server.username, password)
            .build()?;

        let res = client
            .post_json::<Value, ListenerWithEndpoints>("/listeners", &listener)
            .await?;

        if res.status.is_success() {
            create_server_listeners(state.pool.clone(), listener_id, server_id).await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn all_listeners(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ListenerWithEndpoints>> {
    get_listseners(state.pool.clone()).await
}

#[tauri::command]
pub async fn start_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;

        let client = ClientBuilder::new(&server.server)
            .auth(server.server.username, password)
            .build()?;

        let _ = client
            .post::<Value>(&format!("/listeners/{}/start", id))
            .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;

        let client = ClientBuilder::new(&server.server)
            .auth(server.server.username, password)
            .build()?;

        let _ = client
            .post::<Value>(&format!("/listeners/{}/stop", id))
            .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn modify_listener(_state: tauri::State<'_, AppState>) -> Result<()> {
    // TODO: Build out way to modify listener
    Ok(())
}

#[tauri::command]
pub async fn remove_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;

        let client = ClientBuilder::new(&server.server)
            .auth(server.server.username, password)
            .build()?;

        stop_listener(state.clone(), id).await?;

        let _ = client
            .delete::<Value>(&format!("/listeners/{}", id))
            .await?;
    }

    delete_listener(state.pool.clone(), &id).await?;

    Ok(())
}
