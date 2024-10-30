use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServerState;

mod get;
mod post;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new().routes(routes!(get::get_application_roles, post::create_role))
}
