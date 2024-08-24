use crate::models::state::AppState;
use common::crypt::hash::generate_password_hash;
use common::db::user::create_user;
use common::error::Error;
use common::models::user::Credentials;

#[tauri::command]
pub async fn user_create(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    let hash = generate_password_hash(password)?;

    Ok(create_user(
        state.pool.clone(),
        Credentials {
            username: username.to_string(),
            password: hash,
            key: None,
        },
    )
    .await?)
}
