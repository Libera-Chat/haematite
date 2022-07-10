use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_squit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid: &[u8; 3] = line
            .args
            .get(0)
            .ok_or("missing arugment")?
            .as_slice()
            .try_into()
            .map_err(|_| "invalid sid")?;
        network.servers.remove(sid);

        Ok(Outcome::Empty)
    }
}
