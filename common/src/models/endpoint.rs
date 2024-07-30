use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Endpoint {
    pub id: i64,
    pub listener_id: i64,
    pub endpoint: String,
}
