use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Diff, Network};

use crate::line::Error as LineError;
pub use crate::line_handler::{
    ArgumentCountResolver, Handler as LineHandler, HandlerResolver as LineHandlerResolver,
    Resolution as LineHandlerResolution,
};

pub enum Outcome {
    Unhandled,
    Empty,
    Response(Vec<String>),
    State(Vec<Diff>),
}

#[derive(Debug)]
pub enum Error {
    Line(LineError),
    InvalidNumber,
    InvalidDateTime,
    InvalidFormat,
    EmptyArgument,
    UnexpectedValue,
    InvalidState,
    MissingSource,
    ExcessArguments { expected: usize, actual: usize },
    InsufficientArguments { expected: usize, actual: usize },
}

pub trait Handler {
    /// Check if a given `Config` is suitable for this protocol.
    ///
    /// # Arguments
    ///
    /// * `config` - `Config` object to check.
    ///
    /// # Errors
    ///
    /// Errors if `config` isn't suitable for this protocol.
    fn validate_config(&self, config: &Config) -> Result<(), ConfigError>;

    /// Retrieve protocol-specific handshake data to send to our uplink.
    ///
    /// # Arguments
    ///
    /// * `network` - Data about our current network.
    ///
    /// # Errors
    ///
    /// Errors if, for any reason, handshake data cannot be created.
    fn get_burst<'a>(&self, network: &Network, password: &'a str) -> Result<Vec<String>, String>;

    /// Handle a single line of data.
    ///
    /// # Arguments
    ///
    /// * `network` - Data about our current network.
    ///
    /// # Errors
    ///
    /// Errors if a line cannot be handled.
    fn handle(&mut self, network: &Network, line: &[u8]) -> Result<Outcome, Error>;
}

impl From<StateError> for Error {
    fn from(_error: StateError) -> Self {
        Self::InvalidState
    }
}

impl From<LineError> for Error {
    fn from(error: LineError) -> Self {
        Self::Line(error)
    }
}
