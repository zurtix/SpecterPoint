use crate::models::state::AppState;
use common::{crypt::hash::verify_password_hash, error::Error, models::user::User};

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    let user: User = sqlx::query_as(
        r#"
        SELECT username, password
        FROM user
        WHERE username = ?1
        "#,
    )
    .bind(username)
    .fetch_one(&state.pool)
    .await?;

    if verify_password_hash(user.password, password)? {
        Ok(())
    } else {
        Err(Error::Auth)
    }
}
