use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod get;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().routes(routes!(get::health))
}
