use crate::error::Result;
use crate::models::endpoint::Endpoint;
use sqlx::{QueryBuilder, Sqlite, SqlitePool, Transaction};

pub async fn add_endpoints(
    endpoints: Vec<Endpoint>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<()> {
    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints (id, listener_id, endpoint) ");

    endpoint_query_builder.push_values(endpoints, |mut b, endpoint| {
        b.push_bind(endpoint.id)
            .push_bind(endpoint.listener_id)
            .push_bind(endpoint.endpoint);
    });

    endpoint_query_builder
        .build()
        .execute(&mut **transaction)
        .await?;

    Ok(())
}

pub async fn create_endpoints(
    listener_id: i64,
    endpoints: Vec<String>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<()> {
    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints (listener_id, endpoint) ");

    endpoint_query_builder.push_values(endpoints, |mut b, endpoint| {
        b.push_bind(listener_id).push_bind(endpoint);
    });

    endpoint_query_builder
        .build()
        .execute(&mut **transaction)
        .await?;

    Ok(())
}

pub async fn get_endpoints(listener_id: i64, pool: SqlitePool) -> Vec<Endpoint> {
    sqlx::query_as(
        r#"
        SELECT id, listener_id, endpoint FROM endpoints WHERE listener_id = ?1
        "#,
    )
    .bind(listener_id)
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![])
}

pub async fn delete_endpoints(
    listener_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<()> {
    sqlx::query(
        r#"
    DELETE FROM endpoints where listener_id = ?1
    "#,
    )
    .bind(listener_id)
    .execute(&mut **transaction)
    .await?;

    Ok(())
}
