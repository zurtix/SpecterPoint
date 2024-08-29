pub mod agent;
pub mod listener;
pub mod login;
pub mod server;
pub mod user;

use crate::models::state::AppState;

pub fn make_handlers() -> impl Fn(tauri::Invoke<tauri::Wry>) {
    tauri::generate_handler![
        is_setup_required,
        server::add_server,
        server::all_servers,
        server::remove_server,
        listener::add_listener,
        listener::all_listeners,
        listener::remove_listener,
        listener::start_listener,
        listener::stop_listener,
        login::login,
        user::user_create,
        quit
    ]
}

#[tauri::command]
pub async fn is_setup_required(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM users
        "#,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| "Failed to reach database")?;

    Ok(count == 0)
}

#[tauri::command]
fn quit(handle: tauri::AppHandle) {
    handle.exit(0);
}
