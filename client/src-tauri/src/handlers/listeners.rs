use crate::models::state::AppState;
use common::models::listener::CreateListener;
use sqlx::{Execute, QueryBuilder, Sqlite};

#[tauri::command]
pub async fn create_listener(
    state: tauri::State<'_, AppState>,
    create: CreateListener,
) -> Result<(), String> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|_| "Failed to create database transaction")?;

    let listener_id = sqlx::query_as::<_, (i64,)>(
        r#"
    INSERT TO listeners (name, host, port, type)
    VALUES ()
    RETURNING ID
    "#,
    )
    .bind(create.name)
    .bind(create.host)
    .bind(create.port)
    .bind(create.r#type)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| "Failed to create listener transaction")?;

    let mut endpoint_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoints(endpoint)");

    endpoint_query_builder.push_values(create.endpoints, |mut b, endpoints| {
        for endpoint in endpoints {
            b.push_bind(endpoint);
        }
    });
    endpoint_query_builder.push("RETURNING id");

    let endpoint_query = endpoint_query_builder.build().sql();
    let endpoint_ids = sqlx::query_as::<_, (i64,)>(endpoint_query)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|_| "Failed to create endpoints in transaction")?;

    let mut junction_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO endpoint_junction(listener_id, endpoint_id)");

    junction_query_builder.push_values(endpoint_ids, |mut b, id| {
        b.push_bind(listener_id.0);
        b.push_bind(id.0);
    });

    transaction
        .commit()
        .await
        .map_err(|_| "Failed to commit transaction to database")?;

    Ok(())
}
