use haematite_models::network::Network;
use haematite_models::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::state::add_server;
use super::TS6Handler;

pub fn handle(ts6: &mut TS6Handler, network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    let sid = ts6.uplink.take().ok_or(Error::InvalidState)?.decode();
    let server = Server::new(sid.clone(), line.args[0].decode(), line.args[2].decode());
    add_server(network, sid, server)?;

    Ok(Outcome::Empty)
}
