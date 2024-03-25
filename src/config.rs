use std::{collections::HashMap, path::PathBuf, str::FromStr};

use config::Environment;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "RUST_LOG")]
    pub rust_log: String,

    #[serde(rename = "NOTO_EMOJI_METADATA_PATH")]
    pub noto_emoji_metadata_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config_file =
            std::env::var("CONFIG_FILE").unwrap_or_else(|_| "config/config.json".into());

        let s = config::Config::builder()
            .add_source(config::File::with_name("config/default.yaml").required(false))
            // Add in the current environment file
            // Default to 'development' env
            .add_source(config::File::new(&config_file, config::FileFormat::Json).required(false))
            // Add in settings from the environment (with a prefix of APP)
            .add_source(Environment::default())
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| Config::new().unwrap());
