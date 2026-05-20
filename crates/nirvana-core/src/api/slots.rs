use crate::api::domain::Slot;
use crate::api::errors::TrackingError;
use crate::api::NirvanaApi;
use crate::storage::slot_repo::{self, SlotSort};
use crate::api::NirvanaError;

impl NirvanaApi {
    pub fn get_slots(
        &self,
        from: i64,
        to: Option<i64>,
        sort: SlotSort,
    ) -> Result<Vec<Slot>, NirvanaError> {
        let connection_id = self
            .config
            .active_connection
            .ok_or(TrackingError::NoActiveConnection)?;

        let records =
            slot_repo::get_slots(&self.db, connection_id, from, to, sort)?;
        Ok(records.into_iter().map(Slot::from).collect())
    }
}
