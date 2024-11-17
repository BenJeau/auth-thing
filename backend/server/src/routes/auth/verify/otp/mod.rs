use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

pub mod post;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().routes(routes!(post::verify_otp))
}
