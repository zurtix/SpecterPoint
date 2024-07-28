use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum ListenerTypes {
    Http,
    Https,
    Tcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListener {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub r#type: ListenerTypes,
    pub endpoints: Option<Vec<String>>,
}
