use crate::models::state::AppState;
use common::{
    db::listener::{create_listener, delete_listener, get_listseners},
    error::Result,
    models::{
        endpoint::Endpoint,
        listener::{CreateListener, Listener, ListenerBase},
    },
};

#[tauri::command]
pub async fn listener_create(
    state: tauri::State<'_, AppState>,
    create: CreateListener,
) -> Result<()> {
    Ok(create_listener(state.pool.clone(), create).await?)
}

#[tauri::command]
pub async fn listeners(state: tauri::State<'_, AppState>) -> Result<Vec<Listener>> {
    Ok(get_listseners(state.pool.clone()).await?)
}

#[tauri::command]
pub async fn listener_modify(state: tauri::State<'_, AppState>) -> Result<()> {
    // TODO: Build out way to modify listener
    Ok(())
}

#[tauri::command]
pub async fn listener_delete(state: tauri::State<'_, AppState>, id: i64) -> Result<()> {
    Ok(delete_listener(state.pool.clone(), &id).await?)
}
