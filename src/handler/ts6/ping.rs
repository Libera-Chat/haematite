use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_ping(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let me = &network.servers[&network.me];
        Ok(Outcome::Response(vec![format!(
            ":{} PONG {} {}",
            me.sid,
            me.name,
            line.args
                .get(0)
                .ok_or(Error::ExpectedArguments(1))?
                .decode(),
        )]))
    }
}
