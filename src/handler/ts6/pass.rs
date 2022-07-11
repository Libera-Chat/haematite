use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_pass(&mut self, _network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        self.uplink = Some(
            line.args
                .get(3)
                .ok_or(Error::ExpectedArguments(3))?
                .as_slice()
                .try_into()
                .map_err(|_| Error::BadArgument(3))?,
        );
        Ok(Outcome::Empty)
    }
}
