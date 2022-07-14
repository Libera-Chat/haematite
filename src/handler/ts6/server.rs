use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid as _;

use super::util::add_server;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_server(&mut self, network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 3)?;

        let sid = self.uplink.take().ok_or(Error::InvalidState)?;
        let server = Server::new(sid.decode(), line.args[0].decode(), line.args[2].decode());
        add_server(network, sid, server)?;

        Ok(Outcome::Empty)
    }
}
