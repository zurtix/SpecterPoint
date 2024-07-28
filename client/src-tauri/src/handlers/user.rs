use crate::models::state::AppState;

#[tauri::command]
pub async fn create_user(
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
