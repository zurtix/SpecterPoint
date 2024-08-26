use serde::{Deserialize, Serialize};

#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ServerBase {
    pub name: String,
    pub r#type: String,
    pub host: String,
    pub port: u16,
    pub log_port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub server: ServerBase,
}
