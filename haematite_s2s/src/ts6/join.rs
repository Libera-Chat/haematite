use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

//:420AAAABG JOIN 1657651885 #test +
pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let channel = line.args[1].decode();

    Ok(Outcome::State(vec![
        NetDiff::InternalUser(
            uid.clone(),
            UserDiff::Channel(channel.clone(), UserAction::Add),
        ),
        NetDiff::InternalChannel(
            channel,
            ChanDiff::ExternalUser(uid, ChanAction::Add(Membership::new())),
        ),
    ]))
}
