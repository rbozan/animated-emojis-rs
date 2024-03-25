use std::{collections::HashMap, path::PathBuf, str::FromStr};

use config::Environment;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "noto_emoji_metadata_path")]
    pub noto_emoji_metadata_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(Environment::default().try_parsing(true))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| Config::new().unwrap());
