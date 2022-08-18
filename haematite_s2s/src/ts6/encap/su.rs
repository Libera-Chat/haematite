use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

//:00A ENCAP * SU :420AAAAAB
//:00A ENCAP * SU 420AAAAAB :jess
pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..4)?;

    let uid = line.args[2].decode();
    let account = line.args.get(3).map(DecodeHybrid::decode);

    Ok(Outcome::State(vec![NetDiff::InternalUser(
        uid,
        UserDiff::Account(account),
    )]))
}
