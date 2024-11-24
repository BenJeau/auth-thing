use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

pub mod get;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().routes(routes!(get::auth_config))
}
