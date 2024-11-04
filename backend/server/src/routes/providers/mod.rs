use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod delete;
mod get;
mod post;
mod put;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new().routes(routes!(
                get::get_provider,
                put::update_provider,
                delete::delete_provider
            )),
        )
        .routes(routes!(get::get_providers, post::create_provider))
}
