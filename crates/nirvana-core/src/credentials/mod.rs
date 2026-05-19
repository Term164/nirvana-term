use crate::storage::{Database, DbError};

pub(crate) fn get_token(db: &Database, connection_id: i64) -> Result<String, DbError> {
    let token: String = db.conn().query_row(
        "select credential from credentials where connection_id = ?1",
        [connection_id],
        |row| row.get(0),
    )?;
    Ok(token)
}
