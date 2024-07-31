use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ServerBase {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    #[serde(flatten)]
    pub server: ServerBase,
}
