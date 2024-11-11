use axum::{middleware::from_fn_with_state, Router};
use tower::Layer;
use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};
use utoipa_axum::router::OpenApiRouter;

use crate::{auth_middlewares, layers::CommonTowerLayerBuilder, middlewares, ServerState};

mod applications;
mod auth;
mod health;
mod openapi;
mod providers;
mod roles;
mod users;

pub fn openapi_router(state: ServerState) -> OpenApiRouter {
    let stateful_router = OpenApiRouter::new()
        .nest("/applications", applications::router())
        .nest("/providers", providers::router())
        .nest("/roles", roles::router())
        .nest("/users", users::router())
        .route_layer(from_fn_with_state(
            state.clone(),
            auth_middlewares::auth_middleware,
        ))
        .nest("/health", health::router())
        .nest("/auth/applications/:slug", auth::router())
        .with_state(state);

    openapi::axum_openapi_router().merge(stateful_router)
}

fn frontend_router() -> ServeDir<SetStatus<ServeFile>> {
    ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html"))
}

pub fn router(state: ServerState) -> NormalizePath<Router> {
    let (router, api) = openapi_router(state.clone()).split_for_parts();
    let router_with_docs = router.nest("/docs", openapi::router(api));

    let versioned_router =
        Router::new()
            .nest("/api/v1", router_with_docs)
            .route_layer(from_fn_with_state(
                state.clone(),
                middlewares::version_middleware,
            ));

    let router_with_frontend = if state.config.frontend.enabled {
        versioned_router.fallback_service(frontend_router())
    } else {
        versioned_router
    };

    let router = CommonTowerLayerBuilder::new()
        .build()
        .apply_middlewares(router_with_frontend);

    NormalizePathLayer::trim_trailing_slash().layer(router)
}
