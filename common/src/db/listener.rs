use crate::{
    error::Result,
    models::{
        endpoint::Endpoint,
        listener::{Listener, ListenerBaseFull, ListenerFull},
        metadata::Metadata,
    },
};
use sqlx::SqlitePool;

use super::{
    endpoint::{add_endpoints, create_endpoints, delete_endpoints, get_endpoints},
    metadata::{add_metadata, create_metadata, delete_metadata, get_metadata},
};

pub async fn get_listener(pool: SqlitePool, id: &i64) -> Result<ListenerFull> {
    let listener = sqlx::query_as::<_, Listener>(
        r#"
    SELECT id, name, host, type, port, private_key, public_key FROM listeners WHERE id = ?1
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

    let metadata = sqlx::query_as::<_, Metadata>(
        r#"
    SELECT id, listener_id, name, data FROM metadata WHERE listener_id = ?1
    "#,
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    Ok(ListenerFull {
        listener,
        endpoints,
        metadata,
    })
}

pub async fn get_listseners(pool: SqlitePool) -> Result<Vec<ListenerFull>> {
    let listeners = sqlx::query_as::<_, Listener>(
        r#"
    SELECT id, name, host, port, type, private_key, public_key FROM listeners
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut listeners_full = vec![];
    for base in listeners {
        let endpoints = get_endpoints(base.id, pool.clone()).await;
        let metadata = get_metadata(base.id, pool.clone()).await;
        listeners_full.push(ListenerFull {
            listener: base,
            endpoints,
            metadata,
        });
    }

    Ok(listeners_full)
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

pub async fn add_listener(pool: SqlitePool, lstn: ListenerFull) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query(
        r#"
    INSERT INTO listeners (id, name, host, port, type, private_key, public_key) 
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
    "#,
    )
    .bind(lstn.listener.id)
    .bind(lstn.listener.listener.name)
    .bind(lstn.listener.listener.host)
    .bind(lstn.listener.listener.port)
    .bind(lstn.listener.listener.r#type)
    .bind(lstn.listener.listener.private_key)
    .bind(lstn.listener.listener.public_key)
    .execute(&mut *transaction)
    .await?;

    add_endpoints(lstn.endpoints, &mut transaction).await?;
    add_metadata(lstn.metadata, &mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn create_listener(pool: SqlitePool, create: &ListenerBaseFull) -> Result<i64> {
    let mut transaction = pool.begin().await?;

    let listener_id: i64 = sqlx::query_scalar(
        r#"
    INSERT INTO listeners (name, host, port, type, private_key, public_key)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    RETURNING ID
    "#,
    )
    .bind(&create.listener.name)
    .bind(&create.listener.host)
    .bind(create.listener.port)
    .bind(&create.listener.r#type)
    .bind(&create.listener.private_key)
    .bind(&create.listener.public_key)
    .fetch_one(&mut *transaction)
    .await?;

    create_endpoints(listener_id, create.endpoints.clone(), &mut transaction).await?;
    create_metadata(listener_id, create.metadata.clone(), &mut transaction).await?;

    transaction.commit().await?;

    Ok(listener_id)
}

pub async fn delete_listener(pool: SqlitePool, id: i64) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    delete_metadata(id, &mut transaction).await?;
    delete_endpoints(id, &mut transaction).await?;

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
