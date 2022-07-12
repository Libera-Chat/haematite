use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_pass(
        &mut self,
        _network: &mut Network,
        line: &Line,
    ) -> Result<Outcome, &'static str> {
        self.uplink = Some(line.args.get(3).ok_or("missing argument")?.clone());
        Ok(Outcome::Empty)
    }
}
