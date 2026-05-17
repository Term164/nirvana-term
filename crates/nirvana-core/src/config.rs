use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::api::errors::ConfigError;
use crate::paths::AppPaths;

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
