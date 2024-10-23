use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{layers::CommonTowerLayerBuilder, middlewares, ServerState};

pub fn router(state: ServerState) -> Router {
    let router = Router::new().nest(
        "/api/v1",
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route_layer(from_fn_with_state(
                state.clone(),
                middlewares::version_middleware,
            ))
            .with_state(state),
    );

    CommonTowerLayerBuilder::new()
        .build()
        .apply_middlewares(router)
}
