use crate::models::state::AppState;
use common::crypt::encrypt;
use common::error::Error;
use common::models::server::ServerBase;

#[tauri::command]
pub async fn add_server(state: tauri::State<'_, AppState>, srv: ServerBase) -> Result<(), Error> {
    let encrypted_password = encrypt(&state.key, &srv.password)?;
    sqlx::query(
        r#"
        INSERT INTO servers (name, host, port, username, password) VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(srv.name)
    .bind(srv.host)
    .bind(srv.port)
    .bind(srv.username)
    .bind(encrypted_password)
    .execute(&state.pool)
    .await?;

    Ok(())
}
