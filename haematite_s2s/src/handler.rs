pub mod ts6;

use std::ops::RangeFrom;

use haematite_models::network::{Error as StateError, Network};

use crate::config::{Config, Error as ConfigError};
use crate::line::Line;

pub enum Outcome {
    Unhandled,
    Empty,
    Response(Vec<String>),
}

#[derive(Debug)]
pub enum Error {
    MissingSource,
    BadArgument,
    InsufficientArguments(usize, u8),
    ExcessArguments(usize, u8),
    InvalidState,
}

pub trait Handler {
    fn validate_config(&self, config: &Config) -> Result<(), ConfigError>;

    fn get_burst<'a>(
        &self,
        network: &Network,
        password: &'a str,
    ) -> Result<Vec<String>, &'static str>;

    fn handle(&mut self, network: &mut Network, line: Line) -> Result<Outcome, Error>;
}

impl From<StateError> for Error {
    fn from(_error: StateError) -> Self {
        Self::InvalidState
    }
}

struct ArgRange {
    minimum: u8,
    maximum: u8,
}

impl From<u8> for ArgRange {
    fn from(other: u8) -> Self {
        Self {
            minimum: other,
            maximum: other,
        }
    }
}

impl From<RangeFrom<u8>> for ArgRange {
    fn from(other: RangeFrom<u8>) -> Self {
        Self {
            minimum: other.start,
            maximum: u8::MAX,
        }
    }
}

impl Error {
    fn assert_arg_count(line: &Line, expected: impl Into<ArgRange>) -> Result<(), Error> {
        let actual = line.args.len();
        let expected: ArgRange = expected.into();

        if actual < expected.minimum.into() {
            Err(Error::InsufficientArguments(actual, expected.minimum))
        } else if actual > expected.maximum.into() {
            Err(Error::ExcessArguments(actual, expected.maximum))
        } else {
            Ok(())
        }
    }
}
