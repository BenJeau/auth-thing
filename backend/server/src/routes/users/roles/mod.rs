use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod delete;
mod get;
mod post;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new().routes(routes!(post::assign_user_role, delete::remove_user_role)),
        )
        .routes(routes!(get::get_user_roles))
}
