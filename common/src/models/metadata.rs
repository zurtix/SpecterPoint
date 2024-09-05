use serde::{Deserialize, Serialize};

#[derive(Clone, sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub id: i64,
    pub listener_id: i64,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub base: MetadataBase,
}

#[derive(Clone, sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct MetadataBase {
    pub name: String,
    pub data: String,
}
