use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod delete;
mod get;
mod put;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        .nest(
            "/:id",
            OpenApiRouter::new().routes(routes!(
                get::get_role,
                put::update_role,
                delete::delete_role
            )),
        )
        .routes(routes!(get::get_roles))
}
