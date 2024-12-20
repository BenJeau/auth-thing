use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};
use tracing::info;

#[derive(Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn address(&self) -> Result<SocketAddr, AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }
}

#[derive(Deserialize, Clone)]
pub struct Encryption {
    pub server_key: String,
}

#[derive(Deserialize, Clone)]
pub struct Email {
    pub relay: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct Frontend {
    pub enabled: bool,
}

#[derive(Deserialize, Clone)]
pub struct EmailAuth {
    pub resend_min_seconds: u64,
    pub code_length: usize,
}

#[derive(Deserialize, Clone)]
pub struct Auth {
    pub email: EmailAuth,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: Database,
    pub server: Server,
    pub encryption: Encryption,
    #[serde(default)]
    pub email: Option<Email>,
    #[serde(default)]
    pub frontend: Frontend,
    pub auth: Auth,
}

impl Config {
    pub fn new() -> figment::error::Result<Self> {
        info!("Fetching config");

        Figment::new()
            .merge(Toml::string(include_str!("../config.toml")))
            .merge(Env::prefixed("AUTH__").split("__"))
            .extract()
    }
}
