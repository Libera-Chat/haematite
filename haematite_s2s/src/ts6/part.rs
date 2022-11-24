use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use super::util::channel::{ForgetContext, Forgettable as _};
use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(1, 2, Self {})
    }
}

impl LineHandler for Handler {
    //:420AAAABG PART #test
    //:420AAAABG PART #test :foo bar
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
        let channel_name = line.args[0].decode();
        let channel = network
            .channels
            .get(&channel_name)
            .ok_or(StateError::UnknownChannel)?;

        let mut diff = vec![NetDiff::InternalUser(
            uid.clone(),
            UserDiff::Channel(channel_name.clone(), UserAction::Remove),
        )];

        if channel.is_forgettable(ForgetContext::Leave(1)) {
            diff.push(NetDiff::ExternalChannel(channel_name, NetAction::Remove));
        } else {
            diff.push(NetDiff::InternalChannel(
                channel_name,
                ChanDiff::ExternalUser(uid, ChanAction::Remove),
            ));
        }

        Ok(Outcome::State(diff))
    }
}
