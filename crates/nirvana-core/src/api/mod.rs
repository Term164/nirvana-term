mod connection;
pub mod domain;
pub mod errors;
mod info;
mod slots;
mod tracking;

use std::path::PathBuf;

use crate::config::AppConfig;
use crate::paths::AppPaths;
use crate::storage::Database;

use errors::NirvanaError;

pub use crate::storage::slot_repo::SlotSort;

pub struct NirvanaApi {
    paths: AppPaths,
    config: AppConfig,
    db: Database,
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
