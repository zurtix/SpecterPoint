use common::models::user::User;
use sqlx::SqlitePool;

pub async fn get_user(pool: SqlitePool, username: &str) -> User {
    sqlx::query_as::<_, User>(r#"SELECT username, password FROM users WHERE username = ?1"#)
        .bind(username)
        .fetch_one(&pool)
        .await
        .unwrap()
}
