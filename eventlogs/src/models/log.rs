use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub message: String,
    pub timestamp: String,
}
