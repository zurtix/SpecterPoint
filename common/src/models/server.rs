use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ServerSchemes {
    Http,
    Https,
}

#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ServerBase {
    pub name: String,
    pub r#type: String,
    pub scheme: ServerSchemes,
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

pub trait IntoServer {
    fn into(&self) -> String;
}

impl std::fmt::Display for ServerSchemes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = match self {
            ServerSchemes::Https => "https",
            ServerSchemes::Http => "http",
        };
        write!(f, "{}", scheme)
    }
}

impl std::fmt::Display for ServerBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}", self.scheme, self.host, self.port)
    }
}
