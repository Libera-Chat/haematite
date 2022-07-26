use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.get_user_mut(&uid)?;
    user.oper = Some(line.args[0].decode());

    Ok(Outcome::Empty)
}
