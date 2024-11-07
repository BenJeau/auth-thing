use utoipa_axum::router::OpenApiRouter;

use crate::ServerState;

// pub mod get;

mod login;
// mod openid;
mod signup;

pub fn router() -> OpenApiRouter<ServerState> {
    OpenApiRouter::new()
        // .route("/", get(get::get_enabled_auth))
        .nest("/login", login::router())
        // .nest("/openid", openid::router())
        .nest("/signup", signup::router())
}
