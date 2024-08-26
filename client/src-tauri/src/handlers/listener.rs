use crate::models::state::AppState;
use common::{
    db::listener::{create_listener, delete_listener, get_listseners},
    error::Result,
    models::listener::{CreateListener, Listener},
};

#[tauri::command]
pub async fn listener_create(
    state: tauri::State<'_, AppState>,
    create: CreateListener,
    servers: Vec<i64>,
) -> Result<()> {
    let listener_id = create_listener(state.pool.clone(), create).await?;

    // TODO: Make HTTP request to server
    // Once request is successful
    // Add listener server pair into db

    // let client = reqwest::Client::new();
    //     reqwest::
    //     create_server_listeners(state.pool.clone(), listener_id, server_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn listeners(state: tauri::State<'_, AppState>) -> Result<Vec<Listener>> {
    let listeners = get_listseners(state.pool.clone()).await?;
    Ok(listeners)
}

#[tauri::command]
pub async fn listener_modify(state: tauri::State<'_, AppState>) -> Result<()> {
    // TODO: Build out way to modify listener
    Ok(())
}

#[tauri::command]
pub async fn listener_delete(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    delete_listener(state.pool.clone(), &id).await?;
    Ok(())
}
