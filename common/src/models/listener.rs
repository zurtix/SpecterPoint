use crate::models::endpoint::Endpoint;
use crate::models::metadata::Metadata;
use crate::models::metadata::MetadataBase;
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
    pub private_key: String,
    pub public_key: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub id: i64,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub listener: ListenerBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerFull {
    #[serde(flatten)]
    pub inner: Listener,
    pub endpoints: Vec<Endpoint>,
    pub metadata: Vec<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerBaseFull {
    #[serde(flatten)]
    pub listener: ListenerBase,
    pub endpoints: Vec<String>,
    pub metadata: Vec<MetadataBase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerState {
    pub id: i64,
    pub running: bool,
}

impl std::fmt::Display for ListenerFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            self.inner.listener.host, self.inner.listener.port
        )
    }
}
