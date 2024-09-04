use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Metadata {
    pub id: u64,
    pub listener_id: u64,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub base: MetadataBase,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MetadataBase {
    pub name: String,
    pub data: String,
}
