use crate::models::state::AppState;
use common::{
    crypt::aes::decrypt,
    db::{
        listener::{create_listener, delete_listener, get_listener, get_listseners},
        server::{create_server_listeners, get_server, get_server_ids, get_servers},
    },
    error::Result,
    models::listener::{Listener, ListenerBaseWithEndpoints, ListenerWithEndpoints},
};
use serde_json::{json, Value};

#[tauri::command]
pub async fn add_listener(
    state: tauri::State<'_, AppState>,
    create: ListenerBaseWithEndpoints,
    //    server_ids: Vec<i64>,
) -> Result<()> {
    let listener_id = create_listener(state.pool.clone(), &create).await?;
    let server_ids = get_server_ids(state.pool.clone()).await?;
    let listener = get_listener(state.pool.clone(), &listener_id).await?;

    for server_id in server_ids {
        let server = get_server(state.pool.clone(), &server_id).await?;
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;
        let client = crate::httpclient::ClientBuilder::new(
            server.server.username,
            password,
            format!("http://{}:{}", server.server.host, server.server.port),
        )
        .build()
        .await;

        let res = client
            .request::<Value>(reqwest::Method::POST, "/listeners", Some(json!(&listener)))
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
    let listeners = get_listseners(state.pool.clone()).await?;
    Ok(listeners)
}

#[tauri::command]
pub async fn start_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;
        let client = crate::httpclient::ClientBuilder::new(
            server.server.username,
            password,
            format!("http://{}:{}", server.server.host, server.server.port),
        )
        .build()
        .await;

        let _ = client
            .request::<Value>(
                reqwest::Method::POST,
                &format!("/listeners/{}/start", id),
                None,
            )
            .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;
        let client = crate::httpclient::ClientBuilder::new(
            server.server.username,
            password,
            format!("http://{}:{}", server.server.host, server.server.port),
        )
        .build()
        .await;

        let _ = client
            .request::<Value>(
                reqwest::Method::POST,
                &format!("/listeners/{}/stop", id),
                None,
            )
            .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn modify_listener(state: tauri::State<'_, AppState>) -> Result<()> {
    // TODO: Build out way to modify listener
    Ok(())
}

#[tauri::command]
pub async fn remove_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    let servers = get_servers(state.pool.clone()).await?;

    for server in servers {
        let password = decrypt(&state.key.read().unwrap(), &server.server.password)?;
        let client = crate::httpclient::ClientBuilder::new(
            server.server.username,
            password,
            format!("http://{}:{}", server.server.host, server.server.port),
        )
        .build()
        .await;

        let res = client
            .request::<Value>(reqwest::Method::DELETE, &format!("/listeners/{}", id), None)
            .await?;
    }
    Ok(())
}
