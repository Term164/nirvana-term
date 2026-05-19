use crate::paths::AppPaths;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("config parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("config save error: {0}")]
    Save(#[from] toml::ser::Error),
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct AppConfig {
    #[serde(default)]
    pub active_connection: Option<i64>,
}

impl AppConfig {
    pub(crate) fn load(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub(crate) fn save(&self, paths: &AppPaths) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(&paths.config_dir)?;
        fs::write(&paths.config_file, content)?;
        Ok(())
    }
}
