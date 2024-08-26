use crate::app::App;
use axum::routing::Router;

pub mod listeners;
pub mod users;

pub fn routes(state: App) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/listeners", listeners::routes())
                .nest("/users", users::routes()),
        )
        .with_state(state)
}
