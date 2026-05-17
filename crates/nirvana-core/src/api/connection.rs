use crate::api::NirvanaApi;
use crate::api::NirvanaError;
use crate::api::domain::ActiveConnection;
use crate::api::domain::Connection;
use crate::storage::connection_repo;

impl NirvanaApi {
    pub fn list_connections(&self) -> Result<Vec<Connection>, NirvanaError> {
        let records = connection_repo::list(&self.db)?;
        Ok(records.into_iter().map(Connection::from).collect())
    }

    pub fn active_connection(&self) -> Option<&ActiveConnection> {
        self.config.active_connection.as_ref()
    }
}
