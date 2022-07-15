use haematite_models::network::Network;
use haematite_models::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::add_server;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_server(&mut self, network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Line::assert_arg_count(line, 3)?;

        let sid = self.uplink.take().ok_or(Error::InvalidState)?;
        let server = Server {
            id: sid.decode(),
            name: line.args[0].decode(),
            description: line.args[2].decode(),
        };
        add_server(network, sid, server)?;

        Ok(Outcome::Empty)
    }
}
