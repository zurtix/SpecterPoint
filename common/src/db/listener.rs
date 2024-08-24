use crate::{
    error::Result,
    models::{
        endpoint::Endpoint,
        listener::{CreateListener, Listener, ListenerBase},
        Id,
    },
};
use sqlx::{Execute, QueryBuilder, Sqlite, SqlitePool};

pub async fn get_listener(pool: SqlitePool, id: &i64) -> Result<Listener> {
    let base = sqlx::query_as::<_, ListenerBase>(
        r#"SELECT id, name, host, port FROM listeners WHERE id = ?1"#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    let endpoints =
        sqlx::query_as::<_, Endpoint>("SELECT id, endpoint FROM endpoints WHERE listener_id = ?1")
            .bind(id)
            .fetch_all(&pool)
            .await?;

    Ok(Listener {
        listener: base,
        endpoints,
    })
}

pub async fn get_listseners(pool: SqlitePool) -> Result<Vec<Listener>> {
    let listeners_base = sqlx::query_as::<_, ListenerBase>(
        r#"
        SELECT id, name, host, port, type FROM listeners
        "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut listeners = vec![];
    for base in listeners_base {
        let endpoints: Vec<Endpoint> = sqlx::query_as::<_, Endpoint>(
            r#"
            SELECT id, endpoint WHERE listener_id = ?1
            "#,
        )
        .bind(base.id)
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);

        listeners.push(Listener {
            listener: base,
            endpoints,
        });
    }

    Ok(listeners)
}

pub async fn get_listener_ids(pool: SqlitePool) -> Result<Vec<Id>> {
    Ok(sqlx::query_as(r#"SELECT id FROM listeners"#)
        .fetch_all(&pool)
        .await?)
}

pub async fn add_listener(pool: SqlitePool, lstn: Listener) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query(
        r#"
     INSERT INTO listeners (id, name, host, port) VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(lstn.listener.id)
    .bind(lstn.listener.name)
    .bind(lstn.listener.host)
    .bind(lstn.listener.port)
    .execute(&mut *transaction)
    .await?;

    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints(listener_id, endpoint)");

    endpoint_query_builder.push_values(lstn.endpoints, |mut b, endpoint| {
        b.push_bind(endpoint.id);
        b.push_bind(lstn.listener.id);
        b.push_bind(endpoint.endpoint);
    });

    let query = endpoint_query_builder.build().sql();

    sqlx::query(query).execute(&mut *transaction).await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn create_listener(pool: SqlitePool, create: CreateListener) -> Result<()> {
    let mut transaction = pool.begin().await?;

    let listener_id = sqlx::query_as::<_, (i64,)>(
        r#"
    INSERT TO listeners (name, host, port, type)
    VALUES ()
    RETURNING ID
    "#,
    )
    .bind(create.listener.name)
    .bind(create.listener.host)
    .bind(create.listener.port)
    .bind(create.listener.r#type)
    .fetch_one(&mut *transaction)
    .await?;

    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints(listener_id, endpoint)");

    endpoint_query_builder.push_values(create.endpoints, |mut b, endpoint| {
        b.push_bind(listener_id.0);
        b.push_bind(endpoint);
    });
    endpoint_query_builder.push("RETURNING id");

    let endpoint_query = endpoint_query_builder.build().sql();
    sqlx::query(endpoint_query)
        .fetch_all(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn delete_listener(pool: SqlitePool, id: &i64) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query("DELETE FROM endpoints where listener_id = ?1")
        .bind(id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query("DELETE FROM listeners where id = ?1")
        .bind(id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}
