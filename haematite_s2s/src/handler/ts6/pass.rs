use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_pass(&mut self, _network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 4)?;

        self.uplink = Some(line.args[3].clone());
        Ok(Outcome::Empty)
    }
}
