use utoipa_axum::router::OpenApiRouter;

use crate::ServerState;

mod email;
mod otp;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest("/email", email::router())
        .nest("/otp", otp::router())
}
