use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

pub mod get;
pub mod post;
pub mod put;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new().routes(routes!(get::get_user, put::update_user)),
        )
        .routes(routes!(get::get_users, post::create_user))
}
