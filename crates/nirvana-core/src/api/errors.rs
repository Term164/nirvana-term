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

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("db I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("duplicate connection name: '{0}'")]
    DuplicateName(String),
}
