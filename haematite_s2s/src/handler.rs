use haematite_events::EventStore;

use crate::line::Error as LineError;
use crate::util::mode::PairError;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    InvalidProtocol,
    InvalidState,
    MissingSource,
    MissingArgument,

    UnknownBan,
    UnknownChannel,
    UnknownServer,
    UnknownUser,
    UnknownMode,
    UnknownItem,

    EventStore(haematite_events::event_store::Error),
}

impl From<haematite_events::event_store::Error> for Error {
    fn from(value: haematite_events::event_store::Error) -> Self {
        Self::EventStore(value)
    }
}

pub enum Outcome {
    Responses(Vec<String>),
    Unhandled,
    Handled,
}

pub trait Handler {
    /// Retrieve protocol-specific handshake data to send to our uplink.
    ///
    /// # Arguments
    ///
    /// * `network` - Data about our current network.
    ///
    /// # Errors
    ///
    /// Errors if, for any reason, handshake data cannot be created.
    fn get_burst(&self, password: &str) -> Result<Vec<String>, String>;

    /// Handle a single line of data.
    ///
    /// # Arguments
    ///
    /// * `network` - Data about our current network.
    ///
    /// # Errors
    ///
    /// Errors if a line cannot be handled.
    fn handle<E: EventStore>(
        &mut self,
        event_store: &mut E,
        network: &mut Network,
        line: &[u8],
    ) -> Result<Outcome, Error>;
}

impl From<StateError> for Error {
    fn from(_error: StateError) -> Self {
        Self::InvalidState
    }
}

impl From<LineError> for Error {
    fn from(_error: LineError) -> Self {
        Self::InvalidProtocol
    }
}

impl From<PairError> for Error {
    fn from(_error: PairError) -> Self {
        Self::MissingArgument
    }
}
