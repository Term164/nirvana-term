use crate::storage::connection_repo::ConnectionRecord;

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
