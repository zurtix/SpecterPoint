#[derive(sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub data: String,
}
