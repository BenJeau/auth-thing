use std::net::SocketAddr;
use tracing::info;

mod auth_middlewares;
mod config;
mod crypto;
mod error;
mod jwt;
mod layers;
mod middlewares;
mod routes;
mod schemas;
mod state;
mod telemetry;

pub use error::{Error, Result};
pub use state::ServerState;

const ENV_FILTER: &str = "server=debug,tower_http=debug,database=debug";

fn main() {
    telemetry::setup_basic_logging(ENV_FILTER);
    let _guard = telemetry::setup_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            start().await.unwrap();
        });
}

async fn start() -> Result<()> {
    telemetry::setup_tracing(ENV_FILTER).await;

    let state = ServerState::new().await?;
    database::run_migrations(&state.pool).await?;

    let address = state.config.server.address()?;
    info!("Listening on {address}");

    axum::serve(
        tokio::net::TcpListener::bind(address).await?,
        routes::router(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
