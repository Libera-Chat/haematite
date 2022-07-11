use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_server(
        &mut self,
        network: &mut Network,
        line: &Line,
    ) -> Result<Outcome, &'static str> {
        let sid = self.uplink.take().ok_or("invalid state")?;

        network.servers.insert(
            sid,
            Server::new(sid.decode(), line.args[0].decode(), line.args[2].decode()),
        );

        Ok(Outcome::Empty)
    }
}
