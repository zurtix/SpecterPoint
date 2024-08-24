use crate::models::state::AppState;
use common::{
    crypt::hash::verify_password_hash,
    db::user::get_user,
    error::Error,
    models::user::{Credentials, User},
};

#[tauri::command]
pub async fn login(state: tauri::State<'_, AppState>, creds: Credentials) -> Result<(), Error> {
    let user: User = get_user(state.pool.clone(), &creds.username).await?;

    if verify_password_hash(user.password, &creds.password)? {
        if let Some(key) = creds.key {
            state.set_key(key.to_string());
        }
        Ok(())
    } else {
        Err(Error::Auth)
    }
}
