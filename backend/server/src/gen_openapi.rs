#![allow(dead_code)]

mod config;
mod error;
mod layers;
mod middlewares;
mod routes;
mod state;

pub use error::{Error, Result};
pub use state::ServerState;

#[tokio::main]
async fn main() {
    let state = crate::state::ServerState::new().await;
    let openapi = crate::routes::openapi_router(state).into_openapi();
    let doc = openapi.to_pretty_json().unwrap();
    std::fs::write("./openapi.json", doc).unwrap();
}
