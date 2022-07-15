use haematite_models::network::{Error as StateError, Network};

use crate::config::{Config, Error as ConfigError};
use crate::line::Error as LineError;

pub enum Outcome {
    Unhandled,
    Empty,
    Response(Vec<String>),
}

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    InvalidProtocol,
    InvalidState,
    MissingSource,
}

pub trait Handler {
    fn validate_config(&self, config: &Config) -> Result<(), ConfigError>;

    fn get_burst<'a>(
        &self,
        network: &Network,
        password: &'a str,
    ) -> Result<Vec<String>, &'static str>;

    fn handle(&mut self, network: &mut Network, line: &[u8]) -> Result<Outcome, Error>;
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
