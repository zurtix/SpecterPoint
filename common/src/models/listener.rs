use crate::models::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum ListenerTypes {
    Http,
    Https,
    Tcp,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ListenerBase {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub r#type: ListenerTypes,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub id: i64,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub listener: ListenerBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerWithEndpoints {
    #[serde(flatten)]
    pub listener: Listener,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerBaseWithEndpoints {
    #[serde(flatten)]
    pub listener: ListenerBase,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerState {
    pub id: i64,
    pub running: bool,
}
