use crate::{
    error::Result,
    models::user::{BaseCredential, User},
};
use sqlx::SqlitePool;

pub async fn get_user(pool: SqlitePool, username: &str) -> Result<User> {
    Ok(sqlx::query_as::<_, User>(
        r#"
    SELECT id, username, password FROM users WHERE username = ?1
    "#,
    )
    .bind(username)
    .fetch_one(&pool)
    .await?)
}

pub async fn create_user(pool: SqlitePool, user: BaseCredential) -> Result<()> {
    sqlx::query(
        r#"
    INSERT INTO users (username, password) VALUES (?1, ?2)
    "#,
    )
    .bind(user.username)
    .bind(user.password)
    .execute(&pool)
    .await?;

    Ok(())
}
