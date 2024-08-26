use sqlx::SqlitePool;

use crate::{
    error::Result,
    models::server::{Server, ServerBase},
};

pub async fn get_servers(pool: SqlitePool) -> Result<Vec<Server>> {
    Ok(sqlx::query_as(
        r#"SELECT id, name, type, host, port, log_port, username, password FROM servers"#,
    )
    .fetch_all(&pool)
    .await?)
}

pub async fn delete_server(pool: SqlitePool, id: &i64) -> Result<()> {
    sqlx::query(r#"DELETE FROM servers WHERE id = ?1"#)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn create_server(pool: SqlitePool, server: ServerBase, enc_pass: String) -> Result<i64> {
    Ok(sqlx::query_scalar(
        r#"
        INSERT INTO servers (name, type, host, port, log_port, username, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id;
        "#,
    )
    .bind(&server.name)
    .bind(&server.r#type)
    .bind(&server.host)
    .bind(server.port)
        .bind(server.log_port)
    .bind(&server.username)
    .bind(enc_pass)
    .fetch_one(&pool)
    .await?)
}
