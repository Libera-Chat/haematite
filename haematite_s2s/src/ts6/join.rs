use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(3, 3, Self {})
    }
}

impl LineHandler for Handler {
    //:420AAAABG JOIN 1657651885 #test +
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
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
}
