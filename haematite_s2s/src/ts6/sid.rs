use haematite_models::irc::network::Network;
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::state::add_server;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let sid = line.args[2].decode();
    let server = Server::new(sid.clone(), line.args[0].decode(), line.args[3].decode());
    add_server(network, sid, server)?;

    Ok(Outcome::Empty)
}
