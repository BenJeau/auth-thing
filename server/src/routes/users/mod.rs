use axum::{routing::get, Router};

use crate::ServerState;

pub mod get;
pub mod post;
pub mod put;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/:id", get(get::get_user).put(put::update_user))
        .route("/", get(get::get_users).post(post::create_user))
}
