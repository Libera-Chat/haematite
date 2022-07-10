use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_sid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let args: &[Vec<u8>; 4] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing argument")?;
        let sid: [u8; 3] = args[2]
            .as_slice()
            .try_into()
            .map_err(|_| "missing argument")?;

        network.servers.insert(
            sid,
            Server::new(sid.decode(), args[0].decode(), args[3].decode()),
        );

        Ok(Outcome::Empty)
    }
}
