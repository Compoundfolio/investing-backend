use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub auth: AuthSettings,
    pub datasource: Datasources,
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
}

impl Settings {
    pub fn from_config_file(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path))
            .build()?
            .try_deserialize()
    }
}
