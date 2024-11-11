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
pub struct Jwt {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration: u64,
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
pub struct Config {
    pub database: Database,
    pub server: Server,
    pub encryption: Encryption,
    pub jwt: Jwt,
    #[serde(default)]
    pub email: Option<Email>,
    #[serde(default)]
    pub frontend: Frontend,
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
