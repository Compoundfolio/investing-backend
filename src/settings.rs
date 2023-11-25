use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub env_name: String,
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
    pub run_migrations: bool,
}

impl Settings {
    pub fn from_config() -> Result<Self, ConfigError> {
        let env_name = env::var("ENV_NAME").unwrap_or_else(|_| "local".into());
        Config::builder()
            .add_source(File::with_name(&format!("config.{env_name}.toml")))
            .add_source(Environment::with_prefix("app").separator("__"))
            .set_override("env_name", env_name)?
            .build()?
            .try_deserialize()
    }
}
