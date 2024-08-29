use crate::models::state::AppState;
use common::crypt::hash::generate_password_hash;
use common::db::user::create_user;
use common::error::Result;
use common::models::user::BaseCredential;

#[tauri::command]
pub async fn user_create(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
) -> Result<()> {
    let hash = generate_password_hash(password)?;

    create_user(
        state.pool.clone(),
        BaseCredential {
            username: username.to_string(),
            password: hash,
        },
    )
    .await
}
