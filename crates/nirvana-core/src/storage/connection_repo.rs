use crate::api::errors::DbError;
use crate::storage::Database;

pub(crate) fn list(db: &Database) -> Result<Vec<ConnectionRecord>, DbError> {
    let mut stmt = db.conn().prepare(
        "select id, name, kind, base_url, identity, secret_store,
                created_at, updated_at
         from connections
         order by id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ConnectionRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            kind: row.get(2)?,
            base_url: row.get(3)?,
            identity: row.get(4)?,
            secret_store: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    rows.collect::<Result<_, _>>().map_err(DbError::from)
}

pub(crate) struct ConnectionRecord {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub base_url: String,
    pub identity: String,
    pub secret_store: String,
    pub created_at: i64,
    pub updated_at: i64,
}
