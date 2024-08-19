use crate::models::state::AppState;
use common::crypt::aes::encrypt;
use common::error::Error;
use common::models::server::{Server, ServerBase};

#[tauri::command]
pub async fn add_server(
    state: tauri::State<'_, AppState>,
    server: ServerBase,
) -> Result<(), Error> {
    let encrypted_password = encrypt(&state.key.read().unwrap(), &server.password)?;
    sqlx::query(
        r#"
        INSERT INTO servers (name, type, host, port, username, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
    )
    .bind(&server.name)
    .bind(&server.r#type)
    .bind(&server.host)
    .bind(server.port)
    .bind(&server.username)
    .bind(encrypted_password)
    .execute(&state.pool)
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn all_servers(state: tauri::State<'_, AppState>) -> Result<Vec<Server>, Error> {
    Ok(
        sqlx::query_as(r#"SELECT id, name, type, host, port, username, password FROM servers"#)
            .fetch_all(&state.pool)
            .await?,
    )
}
