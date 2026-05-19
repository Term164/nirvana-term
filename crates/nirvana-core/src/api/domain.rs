use crate::storage::connection_repo::ConnectionRecord;
use crate::storage::slot_repo::SlotWithTicket;

#[derive(Debug)]
pub struct Connection {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub host: String,
    pub identity: String,
    pub secret_store: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct ConnectionData {
    pub name: String,
    pub kind: String,
    pub host: String,
    pub identity: String,
    pub secret_store: String,
    pub token: String,
}

impl From<ConnectionRecord> for Connection {
    fn from(r: ConnectionRecord) -> Self {
        Self {
            id: r.id,
            name: r.name,
            kind: r.kind,
            host: r.host,
            identity: r.identity,
            secret_store: r.secret_store,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct Slot {
    pub id: i64,
    pub ticket_key: String,
    pub summary: Option<String>,
    pub note: Option<String>,
    pub started_at: i64,
    pub stopped_at: Option<i64>,
    pub published_at: Option<i64>,
}

impl From<SlotWithTicket> for Slot {
    fn from(r: SlotWithTicket) -> Self {
        Self {
            id: r.id,
            ticket_key: r.ticket_key,
            summary: r.summary,
            note: r.note,
            started_at: r.started_at,
            stopped_at: r.stopped_at,
            published_at: r.published_at,
        }
    }
}
