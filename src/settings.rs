use std::env;

use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub web: WebSettings,
    pub auth: AuthSettings,
    pub datasource: Datasources,
}

#[derive(Debug, Deserialize)]
pub struct WebSettings {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub server_secret: String,
    pub password_salt: String,
    pub google: AuthProviderSettings,
}

#[derive(Debug, Deserialize)]
pub struct AuthProviderSettings {
    pub token_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Datasources {
    pub sql_url: String,
    pub redis_url: String,
    pub run_migrations: bool
}

impl Settings {
    pub fn from_config() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
        Config::builder()
            .add_source(File::with_name(&format!("config.{run_mode}.toml")))
            .add_source(Environment::with_prefix("app").separator("__"))
            .build()?
            .try_deserialize()
    }
}
