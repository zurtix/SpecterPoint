use crate::models::{state::AppState, user::User};
use common::crypt::verify_password_hash;
use common::error::Error;

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
    _key: &str,
) -> Result<(), Error> {
    let user: User = sqlx::query_as(
        r#"
        SELECT username, password, data
        FROM user
        WHERE username = ?1
        "#,
    )
    .bind(username)
    .fetch_one(&state.pool)
    .await?;

    if verify_password_hash(&user.password, password)? {
        Ok(())
    } else {
        Err(Error::Backend("Failed authentication".into()))
    }
}
