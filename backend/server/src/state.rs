use axum::extract::FromRef;
use database::SqlitePool;
use email::Mailer;
use tracing::instrument;

use crate::{config::Config, crypto::Crypto, Result};

#[derive(Clone)]
pub struct ServerState {
    pub pool: SqlitePool,
    pub config: Config,
    pub version: Version,
    pub crypto: Crypto,
    pub mailer: Option<Mailer>,
}

#[derive(Clone)]
pub struct Version {
    pub commit_sha: String,
    pub version: String,
}

impl FromRef<ServerState> for SqlitePool {
    fn from_ref(state: &ServerState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<ServerState> for Config {
    fn from_ref(state: &ServerState) -> Self {
        state.config.clone()
    }
}

impl FromRef<ServerState> for Crypto {
    fn from_ref(state: &ServerState) -> Self {
        state.crypto.clone()
    }
}

impl ServerState {
    #[instrument(skip_all)]
    pub async fn new() -> Result<Self> {
        let config = Config::new().expect("Failed to load config");

        let pool = database::connect_to_db(
            &config.database.url,
            config.database.max_connections,
            config.database.min_connections,
        )
        .await
        .expect("Failed to connect to database");

        #[cfg(debug_assertions)]
        let version = Version {
            commit_sha: "dev".to_string(),
            version: "0.0.0".to_string(),
        };

        #[cfg(not(debug_assertions))]
        let version = Version {
            commit_sha: env!("COMMIT_GIT_SHA").to_string(),
            version: env!("SERVER_VERSION").to_string(),
        };

        let crypto = Crypto::new(&config.encryption.server_key)?;

        let mailer = if let Some(email) = &config.email {
            Some(Mailer::new(&email.relay, &email.username, &email.password)?)
        } else {
            None
        };

        Ok(Self {
            pool,
            config,
            version,
            crypto,
            mailer,
        })
    }
}
