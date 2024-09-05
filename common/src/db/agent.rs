use crate::{error::Result, models::agent::Agent};
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

pub async fn upsert_agent(agent: Agent, pool: SqlitePool) -> Result<()> {
    if existing_agent(&agent.id, pool.clone()).await? {
        update_agent(agent, pool).await?;
    } else {
        add_agent(agent, pool).await?;
    }

    Ok(())
}

pub async fn existing_agent(id: &str, pool: SqlitePool) -> Result<bool> {
    Ok(sqlx::query_scalar::<_, i64>(
        r#"
    SELECT COUNT(*) FROM agents WHERE id = ?1
    "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await?
        != 0)
}

pub async fn add_agent(agent: Agent, pool: SqlitePool) -> Result<()> {
    println!("{:?}", agent);
    sqlx::query(
        r#"
    INSERT INTO agents (id, last_seen)
    VALUES (?1, ?2)
    "#,
    )
    .bind(agent.id)
    .bind(agent.last_seen)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn update_agent(agent: Agent, pool: SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE agents SET last_seen = ?2 WHERE id = ?1
        "#,
    )
    .bind(agent.id)
    .bind(agent.last_seen)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn get_agents(pool: SqlitePool) -> Result<Vec<Agent>> {
    Ok(sqlx::query_as(
        r#"
    SELECT id, last_seen FROM agents
    "#,
    )
    .fetch_all(&pool)
    .await?)
}
