use crate::api::NirvanaApi;
use crate::api::NirvanaError;
use crate::api::domain::{Connection, ConnectionData};
use crate::api::errors::TrackingError;
use crate::credentials;
use crate::integration;
use crate::storage::connection_repo;

impl NirvanaApi {
    pub fn list_connections(&self) -> Result<Vec<Connection>, NirvanaError> {
        let records = connection_repo::list(&self.db)?;
        Ok(records.into_iter().map(Connection::from).collect())
    }

    pub fn get_active_connection(&self) -> Result<Option<Connection>, NirvanaError> {
        let Some(connection_id) = self.config.active_connection else {
            return Ok(None);
        };

        Ok(Some(self.get_connection(connection_id)?))
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

    pub fn test_connection_data(&self, mut connection: ConnectionData) -> Result<(), NirvanaError> {
        connection.host = normalize_host(&connection.host);
        let candidate = Connection {
            id: 0,
            name: connection.name,
            kind: connection.kind,
            host: connection.host,
            identity: connection.identity,
            secret_store: connection.secret_store,
            created_at: 0,
            updated_at: 0,
        };
        let integ = integration::build_integration(&candidate, &connection.token)?;
        integ.test_connection()?;
        Ok(())
    }

    pub fn test_connection(&self) -> Result<(), NirvanaError> {
        let connection_id = self
            .config
            .active_connection
            .ok_or(TrackingError::NoActiveConnection)?;

        let connection = self.get_connection(connection_id)?;
        let token = credentials::get_token(&self.db, connection_id)?;
        let integ = integration::build_integration(&connection, &token)?;
        integ.test_connection()?;
        Ok(())
    }
}

fn normalize_host(url: &str) -> String {
    let s = url.trim();
    let s = s.strip_prefix("https://").unwrap_or(s);
    let s = s.strip_prefix("http://").unwrap_or(s);
    let s = s.strip_suffix('/').unwrap_or(s);
    s.to_string()
}
