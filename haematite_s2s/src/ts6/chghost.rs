use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = &line.args[0];
    let user = network.get_user_mut(uid)?;
    user.host = line.args[1].decode();

    Ok(Outcome::Empty)
}
