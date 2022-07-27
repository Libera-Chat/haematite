use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::util::state::del_user;

//:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.args[0].decode();
    del_user(network, &uid)?;

    Ok(Outcome::Empty)
}
