use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod get;
mod post;
mod put;

mod roles;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new()
                .routes(routes!(get::get_user, put::update_user))
                .nest("/roles", roles::router()),
        )
        .routes(routes!(get::get_users, post::create_user))
}
