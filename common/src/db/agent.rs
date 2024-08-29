use crate::error::Result;
use sqlx::SqlitePool;

pub async fn agent_count(pool: SqlitePool) -> Result<u64> {
    Ok(sqlx::query_scalar(
        r#"
    SELECT COUNT(*) FROM agents
    "#,
    )
    .fetch_one(&pool)
    .await?)
}
