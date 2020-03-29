use config::{Config, ConfigError, File};
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub host: String,
}


#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub actix_level: String,
    pub app_level: String,
}


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub postgres: PostgresConfig,
    pub logging: LoggingConfig,
}


impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // Create a configuration parsing instance
        let mut config_instance = Config::new();

        // Read the configuration file
        config_instance.merge(File::with_name("config"))?;

        config_instance.try_into()
    }
}
