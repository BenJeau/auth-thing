use axum::middleware::from_fn_with_state;
use utoipa_axum::router::OpenApiRouter;

use crate::{auth_middlewares, ServerState};

mod login;
mod signup;
mod verify;

pub fn router(state: ServerState) -> OpenApiRouter<ServerState> {
    let public_routes = OpenApiRouter::new()
        .nest("/login", login::router())
        .nest("/signup", signup::router());

    // TODO: maybe clean this up better so that we don't have multiple auth middlewares declared
    let protected_routes = OpenApiRouter::new()
        .nest("/verify", verify::router())
        .route_layer(from_fn_with_state(state, auth_middlewares::auth_middleware));

    public_routes.merge(protected_routes)
}
