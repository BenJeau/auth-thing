use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

pub mod get;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().nest(
        "/jwks.json",
        OpenApiRouter::new().routes(routes!(get::get_jwks)),
    )
}
