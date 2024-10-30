use axum::{middleware::from_fn_with_state, Router};
use utoipa_axum::router::OpenApiRouter;

use crate::{layers::CommonTowerLayerBuilder, middlewares, ServerState};

mod applications;
mod openapi;
mod roles;
mod users;

pub fn openapi_router(state: ServerState) -> OpenApiRouter {
    let stateful_router = OpenApiRouter::new()
        .nest("/applications", applications::router())
        .nest("/roles", roles::router())
        .nest("/users", users::router())
        .route_layer(from_fn_with_state(
            state.clone(),
            middlewares::version_middleware,
        ))
        .with_state(state);

    openapi::axum_openapi_router().merge(stateful_router)
}

pub fn router(state: ServerState) -> Router {
    let (router, api) = openapi_router(state).split_for_parts();
    let router_with_docs = router.nest("/docs", openapi::router(api));
    let versioned_router = Router::new().nest("/api/v1", router_with_docs);

    CommonTowerLayerBuilder::new()
        .build()
        .apply_middlewares(versioned_router)
}
