use crate::{
    error::Result,
    models::{
        endpoint::Endpoint,
        listener::{Listener, ListenerBaseWithEndpoints, ListenerWithEndpoints},
    },
};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

pub async fn get_listener(pool: SqlitePool, id: &i64) -> Result<ListenerWithEndpoints> {
    let listener = sqlx::query_as::<_, Listener>(
        r#"
    SELECT id, name, host, type, port FROM listeners WHERE id = ?1
    "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    let endpoints = sqlx::query_as::<_, Endpoint>(
        r#"
    SELECT id, listener_id, endpoint FROM endpoints WHERE listener_id = ?1
    "#,
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    Ok(ListenerWithEndpoints {
        listener,
        endpoints,
    })
}

pub async fn get_listseners(pool: SqlitePool) -> Result<Vec<ListenerWithEndpoints>> {
    let listeners = sqlx::query_as::<_, Listener>(
        r#"
    SELECT id, name, host, port, type FROM listeners
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut listeners_with_endpoints = vec![];
    for base in listeners {
        let endpoints: Vec<Endpoint> = sqlx::query_as::<_, Endpoint>(
            r#"
        SELECT id, endpoint WHERE listener_id = ?1
        "#,
        )
        .bind(base.id)
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);

        listeners_with_endpoints.push(ListenerWithEndpoints {
            listener: base,
            endpoints,
        });
    }

    Ok(listeners_with_endpoints)
}

pub async fn get_listener_ids(pool: SqlitePool) -> Result<Vec<i64>> {
    Ok(sqlx::query_scalar(
        r#"
    SELECT id FROM listeners
    "#,
    )
    .fetch_all(&pool)
    .await?)
}

pub async fn add_listener(pool: SqlitePool, lstn: ListenerWithEndpoints) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query(
        r#"
    INSERT INTO listeners (id, name, host, port, type) VALUES (?1, ?2, ?3, ?4, ?5)
    "#,
    )
    .bind(lstn.listener.id)
    .bind(lstn.listener.listener.name)
    .bind(lstn.listener.listener.host)
    .bind(lstn.listener.listener.port)
    .bind(lstn.listener.listener.r#type)
    .execute(&mut *transaction)
    .await?;

    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints (id, listener_id, endpoint) ");

    endpoint_query_builder.push_values(lstn.endpoints, |mut b, endpoint| {
        b.push_bind(endpoint.id)
            .push_bind(lstn.listener.id)
            .push_bind(endpoint.endpoint);
    });

    endpoint_query_builder
        .build()
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn create_listener(pool: SqlitePool, create: &ListenerBaseWithEndpoints) -> Result<i64> {
    let mut transaction = pool.begin().await?;

    let listener_id: i64 = sqlx::query_scalar(
        r#"
    INSERT INTO listeners (name, host, port, type)
    VALUES (?1, ?2, ?3, ?4)
    RETURNING ID
    "#,
    )
    .bind(&create.listener.name)
    .bind(&create.listener.host)
    .bind(create.listener.port)
    .bind(&create.listener.r#type)
    .fetch_one(&mut *transaction)
    .await?;

    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints (listener_id, endpoint) ");

    endpoint_query_builder.push_values(&create.endpoints, |mut b, endpoint| {
        b.push_bind(listener_id).push_bind(endpoint);
    });

    endpoint_query_builder
        .build()
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;

    Ok(listener_id)
}

pub async fn delete_listener(pool: SqlitePool, id: &i64) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query(
        r#"
    DELETE FROM endpoints where listener_id = ?1
    "#,
    )
    .bind(id)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        r#"
    DELETE FROM listeners where id = ?1
    "#,
    )
    .bind(id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
