use haematite_models::channel::Membership;
use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

use super::util::state::add_user_channel;

//:420AAAABG JOIN 1657651885 #test +
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
    let channel = &line.args[1];

    add_user_channel(network, uid.clone(), channel, Membership::new())?;

    Ok(Outcome::Empty)
}
