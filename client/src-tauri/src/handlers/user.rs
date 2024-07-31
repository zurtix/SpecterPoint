use crate::models::state::AppState;
use common::crypt::generate_password_hash;
use common::error::Error;

#[tauri::command]
pub async fn create_user(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    key: &str,
) -> Result<(), Error> {
    let hash = generate_password_hash(password)?;

    sqlx::query(
        r#"
        INSERT INTO user (username, password, data)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(username)
    .bind(hash)
    .bind(key)
    .execute(&state.pool)
    .await?;

    Ok(())
}
