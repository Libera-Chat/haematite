pub mod ts6;

use crate::line::Line;
use crate::network::{Error as StateError, Network};

pub enum Outcome {
    Unhandled,
    Empty,
    Response(Vec<String>),
}

#[derive(Debug)]
pub enum Error {
    MissingSource,
    BadArgument,
    ExpectedArguments(u8),
    InvalidState,
}

pub trait Handler {
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
