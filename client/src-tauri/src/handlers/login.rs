use crate::models::state::AppState;
use common::{crypt::hash::verify_password_hash, error::Error, models::user::User};

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    key: &str,
) -> Result<(), Error> {
    let user: User = sqlx::query_as(
        r#"
        SELECT id, username, password
        FROM users
        WHERE username = ?1
        "#,
    )
    .bind(username)
    .fetch_one(&state.pool)
    .await?;

    if verify_password_hash(user.password, password)? {
        state.set_key(key.to_string());
        Ok(())
    } else {
        Err(Error::Auth)
    }
}
