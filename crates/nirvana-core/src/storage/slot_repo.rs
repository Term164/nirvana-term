use crate::storage::{Database, DbError};

#[allow(dead_code)]
pub(crate) struct SlotRecord {
    pub id: i64,
    pub ticket_id: i64,
    pub connection_id: i64,
    pub note: Option<String>,
    pub started_at: i64,
    pub stopped_at: Option<i64>,
    pub published_at: Option<i64>,
}

#[allow(dead_code)]
pub(crate) struct SlotWithTicket {
    pub id: i64,
    pub ticket_key: String,
    pub summary: Option<String>,
    pub connection_id: i64,
    pub note: Option<String>,
    pub started_at: i64,
    pub stopped_at: Option<i64>,
    pub published_at: Option<i64>,
}

pub(crate) fn find_running(db: &Database) -> Result<Option<SlotRecord>, DbError> {
    db.conn()
        .query_row(
            "select id, ticket_id, connection_id, note, started_at, stopped_at, published_at
             from slots
             where stopped_at is null",
            [],
            |row| {
                Ok(SlotRecord {
                    id: row.get(0)?,
                    ticket_id: row.get(1)?,
                    connection_id: row.get(2)?,
                    note: row.get(3)?,
                    started_at: row.get(4)?,
                    stopped_at: row.get(5)?,
                    published_at: row.get(6)?,
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

pub(crate) fn find_running_with_ticket(db: &Database) -> Result<Option<SlotWithTicket>, DbError> {
    db.conn()
        .query_row(
            "select s.id, t.ticket_key, t.summary, s.connection_id, s.note, s.started_at, s.stopped_at, s.published_at
             from slots s
             join tickets t on t.id = s.ticket_id
             where s.stopped_at is null",
            [],
            |row| {
                Ok(SlotWithTicket {
                    id: row.get(0)?,
                    ticket_key: row.get(1)?,
                    summary: row.get(2)?,
                    connection_id: row.get(3)?,
                    note: row.get(4)?,
                    started_at: row.get(5)?,
                    stopped_at: row.get(6)?,
                    published_at: row.get(7)?,
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

pub(crate) fn stop_by_id(db: &Database, slot_id: i64, stopped_at: i64) -> Result<(), DbError> {
    db.conn().execute(
        "update slots set stopped_at = ?1 where id = ?2",
        (stopped_at, slot_id),
    )?;
    Ok(())
}

pub(crate) fn stop_running(db: &Database, stopped_at: i64) -> Result<SlotWithTicket, DbError> {
    let slot = find_running_with_ticket(db)?
        .ok_or(DbError::Sqlite(rusqlite::Error::QueryReturnedNoRows))?;
    if stopped_at <= slot.started_at {
        return Err(DbError::StopBeforeStart);
    }
    stop_by_id(db, slot.id, stopped_at)?;
    Ok(slot)
}

pub(crate) fn insert(
    db: &Database,
    ticket_id: i64,
    connection_id: i64,
    note: Option<&str>,
    started_at: i64,
) -> Result<SlotWithTicket, DbError> {
    db.conn().execute(
        "insert into slots (ticket_id, connection_id, note, started_at, stopped_at, published_at)
         values (?1, ?2, ?3, ?4, null, null)",
        (ticket_id, connection_id, note, started_at),
    )?;

    let slot_id = db.conn().last_insert_rowid();

    db.conn()
        .query_row(
            "select s.id, t.ticket_key, t.summary, s.connection_id, s.note, s.started_at, s.stopped_at, s.published_at
             from slots s
             join tickets t on t.id = s.ticket_id
             where s.id = ?1",
            [slot_id],
            |row| {
                Ok(SlotWithTicket {
                    id: row.get(0)?,
                    ticket_key: row.get(1)?,
                    summary: row.get(2)?,
                    connection_id: row.get(3)?,
                    note: row.get(4)?,
                    started_at: row.get(5)?,
                    stopped_at: row.get(6)?,
                    published_at: row.get(7)?,
                })
            },
        )
        .map_err(DbError::from)
}
