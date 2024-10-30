use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod delete;
mod get;
mod post;
mod put;

mod roles;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new()
                .routes(routes!(
                    get::get_application,
                    put::update_application,
                    delete::delete_application
                ))
                .nest("/roles", roles::router()),
        )
        .routes(routes!(get::get_applications, post::create_application))
}
