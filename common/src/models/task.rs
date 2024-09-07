use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub agent_id: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}
