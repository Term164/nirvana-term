use crate::api::NirvanaApi;
use crate::api::NirvanaError;
use crate::api::domain::{Connection, ConnectionData};
use crate::storage::connection_repo;

impl NirvanaApi {
    pub fn list_connections(&self) -> Result<Vec<Connection>, NirvanaError> {
        let records = connection_repo::list(&self.db)?;
        Ok(records.into_iter().map(Connection::from).collect())
    }

    pub fn active_connection(&self) -> Option<i64> {
        self.config.active_connection
    }

    pub fn set_active_connection(&mut self, id: i64) -> Result<(), NirvanaError> {
        self.config.active_connection = Some(id);
        self.config.save(&self.paths)?;
        Ok(())
    }

    pub fn add_connection(
        &self,
        mut connection: ConnectionData,
    ) -> Result<Connection, NirvanaError> {
        connection.host = normalize_host(&connection.host);
        let record = connection_repo::add(&self.db, connection)?;
        Ok(record.into())
    }
}

fn normalize_host(url: &str) -> String {
    let s = url.trim();
    let s = s.strip_prefix("https://").unwrap_or(s);
    let s = s.strip_prefix("http://").unwrap_or(s);
    let s = s.strip_suffix('/').unwrap_or(s);
    s.to_string()
}
