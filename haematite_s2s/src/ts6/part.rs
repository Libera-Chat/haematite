use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

use super::util::state::del_user_channel;

//:420AAAABG PART #test
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
    let channel_name = &line.args[0];
    del_user_channel(network, uid, channel_name)?;

    Ok(Outcome::Empty)
}
