use haematite_models::network::Network;
use haematite_models::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::add_server;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_sid(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 4)?;

        let sid = &line.args[2];
        let server = Server {
            id: sid.decode(),
            name: line.args[0].decode(),
            description: line.args[3].decode(),
        };
        add_server(network, sid.clone(), server)?;

        Ok(Outcome::Empty)
    }
}
