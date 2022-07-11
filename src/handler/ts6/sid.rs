use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_sid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() != 4 {
            return Err("unexpected argument count");
        }
        let sid: [u8; 3] = line.args[2]
            .as_slice()
            .try_into()
            .map_err(|_| "invalid sid")?;

        network.servers.insert(
            sid,
            Server::new(sid.decode(), line.args[0].decode(), line.args[3].decode()),
        );

        Ok(Outcome::Empty)
    }
}
