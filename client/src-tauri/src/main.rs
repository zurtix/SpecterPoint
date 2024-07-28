// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod handlers;
pub mod models;
pub mod sqlite;

#[tokio::main]
async fn main() {
    sqlite::init().await;

    let pool = sqlite::connect().await;
    let state = models::state::AppState::new(pool).await;

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(handlers::make_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
