use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let oper = Some(line.args[0].decode());

    Ok(Outcome::State(vec![NetDiff::InternalUser(
        uid,
        UserDiff::Oper(oper),
    )]))
}
