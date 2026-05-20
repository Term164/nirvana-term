use crate::api::NirvanaApi;
use crate::api::domain::{PublishFailure, PublishResult};
use crate::api::errors::TrackingError;
use crate::credentials;
use crate::integration;
use crate::storage::slot_repo;

impl NirvanaApi {
    pub fn publish(&self, from: i64, to: Option<i64>) -> Result<PublishResult, super::NirvanaError> {
        let connection_id = self
            .config
            .active_connection
            .ok_or(TrackingError::NoActiveConnection)?;

        let connection = self.get_connection(connection_id)?;
        let token = credentials::get_token(&self.db, connection_id)?;
        let integ = integration::build_integration(&connection, &token)?;

        let slots = slot_repo::get_unpublished(&self.db, connection_id, from, to)?;
        if slots.is_empty() {
            return Ok(PublishResult {
                published_count: 0,
                failed: vec![],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        let now = chrono::Utc::now().timestamp();
        let mut published_ids = Vec::new();
        let mut failed = Vec::new();

        for slot in &slots {
            let duration = slot.stopped_at.unwrap() - slot.started_at;
            match integ.publish_slot(&slot.ticket_key, slot.started_at, duration) {
                Ok(()) => published_ids.push(slot.id),
                Err(e) => failed.push(PublishFailure {
                    ticket_key: slot.ticket_key.clone(),
                    error: e.to_string(),
                }),
            }
        }

        if !published_ids.is_empty() {
            slot_repo::mark_published(&self.db, &published_ids, now)?;
        }

        Ok(PublishResult {
            published_count: published_ids.len(),
            failed,
            timestamp: now,
        })
    }
}
