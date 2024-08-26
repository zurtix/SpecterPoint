use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_port")]
    pub log_port: u16,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_log_port() -> u16 {
    8081
}
