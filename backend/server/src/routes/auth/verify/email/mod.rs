use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod post;

mod resend;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .routes(routes!(post::verify_email))
        .nest("/resend", resend::router())
}
