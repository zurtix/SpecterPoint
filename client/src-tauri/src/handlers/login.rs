use crate::models::{state::AppState, user::User};

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    _key: &str,
) -> Result<(), String> {
    let user: User = sqlx::query_as(
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
