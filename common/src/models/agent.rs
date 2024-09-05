use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    pub id: String,
    pub last_seen: String,
}
