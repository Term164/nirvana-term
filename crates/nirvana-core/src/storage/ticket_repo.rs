use crate::storage::{Database, DbError};

#[allow(unused)]
pub(crate) struct TicketRecord {
    pub id: i64,
    pub ticket_key: String,
    pub summary: Option<String>,
    pub connection_id: i64,
    pub last_worked_at: i64,
}

pub(crate) fn find_by_key(
    db: &Database,
    ticket_key: &str,
    connection_id: i64,
) -> Result<Option<TicketRecord>, DbError> {
    db.conn()
        .query_row(
            "select id, ticket_key, summary, connection_id, last_worked_at
             from tickets
             where ticket_key = ?1 and connection_id = ?2",
            (ticket_key, connection_id),
            |row| {
                Ok(TicketRecord {
                    id: row.get(0)?,
                    ticket_key: row.get(1)?,
                    summary: row.get(2)?,
                    connection_id: row.get(3)?,
                    last_worked_at: row.get(4)?,
                })
            },
        )
        .map(Some)
        .or_else(|e| {
            if e == rusqlite::Error::QueryReturnedNoRows {
                Ok(None)
            } else {
                Err(DbError::from(e))
            }
        })
}

pub(crate) fn insert(
    db: &Database,
    ticket_key: &str,
    summary: Option<&str>,
    connection_id: i64,
    last_worked_at: i64,
) -> Result<TicketRecord, DbError> {
    db.conn().execute(
        "insert into tickets (ticket_key, summary, connection_id, last_worked_at) values (?1, ?2, ?3, ?4)",
        (ticket_key, summary, connection_id, last_worked_at),
    )?;

    Ok(TicketRecord {
        id: db.conn().last_insert_rowid(),
        ticket_key: ticket_key.to_uppercase(),
        summary: summary.map(|s| s.to_string()),
        connection_id,
        last_worked_at,
    })
}

pub(crate) fn touch_last_worked(
    db: &Database,
    ticket_id: i64,
    timestamp: i64,
) -> Result<(), DbError> {
    db.conn().execute(
        "update tickets set last_worked_at = ?1 where id = ?2",
        (timestamp, ticket_id),
    )?;
    Ok(())
}

#[allow(unused)]
pub(crate) fn list_by_connection(
    db: &Database,
    connection_id: i64,
) -> Result<Vec<TicketRecord>, DbError> {
    let mut stmt = db.conn().prepare(
        "select id, ticket_key, summary, connection_id, last_worked_at
         from tickets
         where connection_id = ?1
         order by last_worked_at desc",
    )?;
    let rows = stmt.query_map([connection_id], |row| {
        Ok(TicketRecord {
            id: row.get(0)?,
            ticket_key: row.get(1)?,
            summary: row.get(2)?,
            connection_id: row.get(3)?,
            last_worked_at: row.get(4)?,
        })
    })?;
    rows.collect::<Result<_, _>>().map_err(DbError::from)
}
