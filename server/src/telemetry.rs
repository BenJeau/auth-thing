use sentry::ClientInitGuard;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_basic_logging(env_filter: impl Into<String>) {
    let env_filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| env_filter.into().into());
    let default_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(default_layer)
        .init();
}

pub fn setup_sentry() -> ClientInitGuard {
    info!("Setting up Sentry");

    #[cfg(debug_assertions)]
    let release = "dev".to_string();

    #[cfg(not(debug_assertions))]
    let release = env!("COMMIT_GIT_SHA").to_string();

    sentry::init(sentry::ClientOptions {
        release: Some(release.into()),
        ..Default::default()
    })
}

pub async fn setup_tracing(env_filter: impl Into<String>) {
    info!("Setting up tracing");

    let env_filter: String = env_filter.into();

    let env_filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| env_filter.clone().into());
    let default_layer = tracing_subscriber::fmt::layer();
    let sentry_layer = sentry_tracing::layer();

    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(default_layer)
        .with(sentry_layer)
        .set_default();

    info!(env_filter, "Telemetry setup complete");
}
