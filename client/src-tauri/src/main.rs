// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod handlers;
pub mod manager;
pub mod models;

use common::db::sqlite;
use manager::TcpManager;

const DB_URL: &str = "sqlite://specterpoint-client.db";

#[tokio::main]
async fn main() {
    sqlite::init(DB_URL, Some("./migrations")).await;

    let pool = sqlite::connect(DB_URL).await;
    let manager = TcpManager::new();
    let state = models::state::AppState::new(pool, manager);

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(handlers::make_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
