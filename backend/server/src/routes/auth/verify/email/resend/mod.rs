use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod post;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().routes(routes!(post::resend_verification_code))
}
