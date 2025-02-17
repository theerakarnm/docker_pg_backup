use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub container_name: String,
    pub db_name: String,
    pub db_user: String,
    pub backup_dir: String,
    pub s3_bucket: String,
    pub aws_region: String,
    pub encryption_key: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        config.try_deserialize()
    }
}
