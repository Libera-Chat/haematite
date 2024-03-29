use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let away = line.args.get(0).map(DecodeHybrid::decode);

    Ok(Outcome::State(vec![NetDiff::InternalUser(
        uid,
        UserDiff::Away(away),
    )]))
}
