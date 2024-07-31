use crate::models::state::AppState;
use common::models::{
    endpoint::Endpoint,
    listener::{CreateListener, Listener, ListenerBase},
};
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
    .bind(create.listener.name)
    .bind(create.listener.host)
    .bind(create.listener.port)
    .bind(create.listener.r#type)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| "Failed to create listener transaction")?;

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
        .await
        .map_err(|_| "Failed to create endpoints in transaction")?;

    transaction
        .commit()
        .await
        .map_err(|_| "Failed to commit transaction to database")?;

    Ok(())
}

#[tauri::command]
pub async fn get_all_listeners(state: tauri::State<'_, AppState>) -> Result<Vec<Listener>, String> {
    let listeners_base = sqlx::query_as::<_, ListenerBase>(
        r#"
        SELECT id, name, host, port, type FROM listeners
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| "Failed to obtain all listeners")?;

    let mut listeners = vec![];
    for base in listeners_base {
        let endpoints: Vec<Endpoint> = sqlx::query_as::<_, Endpoint>(
            r#"
            SELECT id, endpoint WHERE listener_id = ?1
            "#,
        )
        .bind(base.id)
        .fetch_all(&state.pool)
        .await
        .unwrap_or(vec![]);

        listeners.push(Listener {
            listener: base,
            endpoints,
        });
    }

    Ok(listeners)
}

#[tauri::command]
pub async fn modify_listener(state: tauri::State<'_, AppState>) -> Result<(), String> {
    // TODO: Build out way to modify listener
    Ok(())
}

#[tauri::command]
pub async fn delete_listener(state: tauri::State<'_, AppState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM listeners WERE id = ?1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| "Failed to delete listener")?;

    Ok(())
}
