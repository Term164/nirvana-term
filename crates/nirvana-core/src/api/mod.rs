mod connection;
pub mod domain;
pub mod errors;
mod info;

use std::path::PathBuf;

use crate::config::AppConfig;
use crate::paths::AppPaths;
use crate::storage::Database;

use errors::{ConfigError, DbError};

pub use domain::{ActiveConnection, Connection};

pub struct NirvanaApi {
    paths: AppPaths,
    config: AppConfig,
    db: Database,
}

#[derive(Debug, thiserror::Error)]
pub enum NirvanaError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("database error: {0}")]
    Db(#[from] DbError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct AppInfo {
    pub version: String,
    pub config_file: PathBuf,
    pub db_file: PathBuf,
    pub log_file: PathBuf,
    pub is_dev: bool,
}

impl NirvanaApi {
    pub fn new() -> Result<Self, NirvanaError> {
        let paths = AppPaths::resolve();
        let config = AppConfig::load(&paths.config_file)?;
        let db = Database::initialize(&paths.db_file)?;
        Ok(Self { paths, config, db })
    }
}
