use crate::models::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum ListenerTypes {
    Http,
    Https,
    Tcp,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ListenerBase {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub r#type: ListenerTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listener {
    #[serde(flatten)]
    pub listener: ListenerBase,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListener {
    #[serde(flatten)]
    pub listener: ListenerBase,
    pub endpoints: Vec<String>,
}
