use crate::api::NirvanaApi;
use crate::api::domain::{Connection, Slot, Ticket};
use crate::api::errors::{IntegrationError, TrackingError};
use crate::credentials;
use crate::integration;
use crate::storage::{DbError, connection_repo, slot_repo, ticket_repo};

impl NirvanaApi {
    pub fn start_slot(
        &self,
        ticket_key: &str,
        at: Option<i64>,
        note: Option<&str>,
    ) -> Result<Slot, super::NirvanaError> {
        let connection_id = self
            .config
            .active_connection
            .ok_or(TrackingError::NoActiveConnection)?;

        let connection = self.get_connection(connection_id)?;
        let ticket_key = ticket_key.to_uppercase();
        let now = at.unwrap_or_else(|| chrono::Utc::now().timestamp());

        let ticket = match ticket_repo::find_by_key(&self.db, &ticket_key, connection_id)? {
            Some(t) => {
                ticket_repo::touch_last_worked(&self.db, t.id, now)?;
                t
            }
            None => {
                let token = credentials::get_token(&self.db, connection_id)?;
                let integ = integration::build_integration(&connection, &token)?;
                let issue = integ.fetch_issue(&ticket_key).map_err(|e| match e {
                    IntegrationError::TicketNotFound(key) => {
                        super::NirvanaError::Tracking(TrackingError::TicketNotFound(key))
                    }
                    other => super::NirvanaError::Integration(other),
                })?;
                ticket_repo::insert(
                    &self.db,
                    &ticket_key,
                    Some(&issue.summary),
                    connection_id,
                    now,
                )?
            }
        };

        if let Some(running) = slot_repo::find_running(&self.db)? {
            let stopped_at = std::cmp::max(running.started_at + 1, now);
            slot_repo::stop_by_id(&self.db, running.id, stopped_at)?;
        }

        let result = slot_repo::insert(&self.db, ticket.id, connection_id, note, now)?;
        let integ = integration::Integration::build_for_url(&connection)?;
        let issue_url = Some(integ.get_issue_link(&result.ticket_key));
        Ok(Slot::from_record(result, issue_url))
    }

    pub fn stop_slot(&self, at: Option<i64>) -> Result<Option<Slot>, super::NirvanaError> {
        let now = at.unwrap_or_else(|| chrono::Utc::now().timestamp());
        match slot_repo::stop_running(&self.db, now) {
            Ok(slot) => {
                let connection = self.get_connection(slot.connection_id)?;
                let integ = integration::Integration::build_for_url(&connection)?;
                let issue_url = Some(integ.get_issue_link(&slot.ticket_key));
                let mut s = Slot::from_record(slot, issue_url);
                s.stopped_at = Some(now);
                Ok(Some(s))
            }
            Err(DbError::Sqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(super::NirvanaError::Db(e)),
        }
    }

    pub(crate) fn get_connection(&self, id: i64) -> Result<Connection, super::NirvanaError> {
        let records = connection_repo::list(&self.db)?;
        records
            .into_iter()
            .find(|r| r.id == id)
            .map(Connection::from)
            .ok_or(super::NirvanaError::Tracking(
                TrackingError::NoActiveConnection,
            ))
    }

    pub fn list_recent_tickets(&self) -> Result<Vec<Ticket>, super::NirvanaError> {
        let connection_id = self
            .config
            .active_connection
            .ok_or(TrackingError::NoActiveConnection)?;
        let records = ticket_repo::list_by_connection(&self.db, connection_id)?;
        Ok(records.into_iter().map(Ticket::from).collect())
    }
}
