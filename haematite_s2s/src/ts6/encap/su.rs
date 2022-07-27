use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

//:00A ENCAP * SU :420AAAAAB
//:00A ENCAP * SU 420AAAAAB :jess
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..4)?;

    let uid = line.args[2].decode();
    let user = network.get_user_mut(&uid)?;
    user.account = line.args.get(3).map(DecodeHybrid::decode);

    Ok(Outcome::Empty)
}
