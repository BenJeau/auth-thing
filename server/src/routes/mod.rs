use axum::{routing::get, Router};

use crate::ServerState;

pub fn router(state: ServerState) -> Router {
    let router = Router::new().nest(
        "/api/v1",
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .with_state(state),
    );

    router
}
