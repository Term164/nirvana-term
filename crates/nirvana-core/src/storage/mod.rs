pub(crate) mod connection_repo;
pub(crate) mod slot_repo;
pub(crate) mod ticket_repo;

use rusqlite::Connection as SqliteConnection;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("db I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("duplicate connection name: '{0}'")]
    DuplicateName(String),
    #[error("stop time must be after the slot start time")]
    StopBeforeStart,
}

pub(crate) struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub(crate) fn conn(&self) -> &SqliteConnection {
        &self.connection
    }

    pub fn initialize(path: &Path) -> Result<Self, DbError> {
        let db = Database::open(path)?;
        db.run_migrations()?;
        Ok(db)
    }

    pub fn open(path: &Path) -> Result<Self, DbError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = SqliteConnection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        Ok(Self { connection: conn })
    }

    pub fn run_migrations(&self) -> Result<(), DbError> {
        const MIGRATIONS: &[(i64, &str)] = &[
            (1, include_str!("migrations/0001_init.sql")),
            (2, include_str!("migrations/0002_tickets_slots.sql")),
        ];

        let user_version: i64 = self.connection.query_row(
            "select user_version from pragma_user_version",
            [],
            |row| row.get(0),
        )?;

        for &(version, sql) in MIGRATIONS {
            if user_version < version {
                self.connection.execute_batch(sql)?;
                self.connection
                    .pragma_update(None, "user_version", version)?;
            }
        }

        Ok(())
    }
}
