use sqlx::SqlitePool;

use crate::{
    error::Result,
    models::server::{Server, ServerBase},
};

pub async fn get_servers(pool: SqlitePool) -> Result<Vec<Server>> {
    Ok(
        sqlx::query_as(r#"SELECT id, name, type, host, port, username, password FROM servers"#)
            .fetch_all(&pool)
            .await?,
    )
}

pub async fn create_server(pool: SqlitePool, server: ServerBase, enc_pass: String) -> Result<()> {
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
    .bind(enc_pass)
    .execute(&pool)
    .await?;

    Ok(())
}
