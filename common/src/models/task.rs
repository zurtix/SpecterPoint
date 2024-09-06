use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub agent_id: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}
