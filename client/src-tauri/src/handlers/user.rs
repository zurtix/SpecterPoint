use crate::models::state::AppState;
use common::crypt::hash::generate_password_hash;
use common::error::Error;

#[tauri::command]
pub async fn create_user(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    let hash = generate_password_hash(password)?;

    sqlx::query(
        r#"
        INSERT INTO user (username, password)
        VALUES (?1, ?2)
        "#,
    )
    .bind(username)
    .bind(hash)
    .execute(&state.pool)
    .await?;

    Ok(())
}
