use crate::app::App;
use axum::{extract::State, response::IntoResponse, Json};
use common::{db::listener::get_listener_ids, error::Result, models::listener::ListenerState};
use comms::debug;

pub async fn get_listeners(State(state): State<App>) -> Result<impl IntoResponse> {
    debug!("Getting all listeners");
    let listeners = get_listener_ids(state.pool).await?;

    let mut states = vec![];

    for id in listeners {
        states.push(ListenerState {
            id,
            running: state.orch.is_running(&id).await,
        })
    }

    Ok(Json(states))
}
