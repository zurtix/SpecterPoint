use crate::app::App;
use axum::routing::Router;

pub mod listeners;
pub mod tasks;
pub mod users;

pub fn routes(state: App) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/listeners", listeners::routes())
                .nest("/users", users::routes())
                .nest("/tasks", tasks::routes()),
        )
        .with_state(state)
}
