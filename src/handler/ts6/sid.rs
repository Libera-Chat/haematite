use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid as _;

use super::util::add_server;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_sid(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() != 4 {
            return Err(Error::ExpectedArguments(4));
        }

        let sid = &line.args[2];
        let server = Server::new(sid.decode(), line.args[0].decode(), line.args[3].decode());
        add_server(network, sid.clone(), server)?;

        Ok(Outcome::Empty)
    }
}
