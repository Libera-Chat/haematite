use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    Ok(Outcome::State(vec![NetDiff::InternalUser(
        line.args[0].decode(),
        UserDiff::Host(line.args[1].decode()),
    )]))
}
