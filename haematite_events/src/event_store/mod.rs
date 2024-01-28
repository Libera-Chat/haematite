pub mod json;
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
}

pub trait EventStore {
    /// Put an event into an event store, to later be exported via an Event Handler.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Type name of this event.
    /// - `event` - The body of this event.
    ///
    /// # Errors
    ///
    /// Errors if this event could not be put in the event store.
    fn store<S: Serialize>(&mut self, event_type: &'static str, event: S) -> Result<(), Error>;
}
