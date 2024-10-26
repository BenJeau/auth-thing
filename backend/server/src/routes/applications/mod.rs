use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

pub mod get;
pub mod post;
pub mod put;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new().routes(routes!(get::get_application, put::update_application)),
        )
        .routes(routes!(get::get_applications, post::create_application))
}
