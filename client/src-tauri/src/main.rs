// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod state;

use state::AppState;

#[tauri::command]
async fn login(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    _key: &str,
) -> Result<(), String> {
    let user: models::user::User = sqlx::query_as(
        r#"
        SELECT username, password, data
        FROM user
        WHERE username = ?1
        "#,
    )
    .bind(username)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| "Failed authentication")?;

    if password != user.password {
        Err("Failed authentication".into())
    } else {
        Ok(())
    }
}

#[tauri::command]
async fn setup(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    key: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT INTO user (username, password, data)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(username)
    .bind(password)
    .bind(key)
    .execute(&state.pool)
    .await
    .map_err(|_| "Failed to create user")?;

    Ok(())
}

#[tauri::command]
async fn setup_required(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM users
        "#,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| "Failed to reach database")?;

    Ok(count != 0)
}

#[tokio::main]
async fn main() {
    let state = state::AppState::new().await;

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![setup_required, setup, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
