use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_squit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid = line.args.get(0).ok_or("missing argument")?.as_slice();
        network.servers.remove(sid);

        Ok(Outcome::Empty)
    }
}
