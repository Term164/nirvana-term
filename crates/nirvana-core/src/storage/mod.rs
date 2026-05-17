pub(crate) mod connection_repo;

use std::{fs, path::Path};

use rusqlite::Connection as SqliteConnection;

use crate::api::errors::DbError;

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
        const MIGRATIONS: &[(i64, &str)] = &[(1, include_str!("migrations/0001_init.sql"))];

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
