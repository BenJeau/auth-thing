use axum::extract::FromRef;
use database::SqlitePool;
use tracing::instrument;

use crate::config::Config;

#[derive(Clone)]
pub struct ServerState {
    pub pool: SqlitePool,
    pub config: Config,
    pub version: Version,
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

impl ServerState {
    #[instrument(skip_all)]
    pub async fn new() -> Self {
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
            version: "v0.0.0".to_string(),
        };

        #[cfg(not(debug_assertions))]
        let version = Version {
            commit_sha: env!("COMMIT_GIT_SHA").to_string(),
            version: env!("SERVER_VERSION").to_string(),
        };

        Self {
            pool,
            config,
            version,
        }
    }
}
