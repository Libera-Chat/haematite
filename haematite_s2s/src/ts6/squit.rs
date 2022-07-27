use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::state::del_server;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let sid = line.args[0].decode();
    del_server(network, &sid)?;

    Ok(Outcome::Empty)
}
