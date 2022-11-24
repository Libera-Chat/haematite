use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};

use super::util::channel::{ForgetContext, Forgettable as _};
use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(0, 1, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
        let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

        let mut diff = Vec::new();

        for channel_name in &user.channels {
            // this .ok_or() shouldn't be needed.
            // we've got a state desync if it is ever hit
            let channel = network
                .channels
                .get(channel_name)
                .ok_or(StateError::UnknownChannel)?;
            diff.push(if channel.is_forgettable(ForgetContext::Leave(1)) {
                NetDiff::ExternalChannel(channel_name.clone(), NetAction::Remove)
            } else {
                NetDiff::InternalChannel(
                    channel_name.clone(),
                    ChanDiff::ExternalUser(uid.clone(), ChanAction::Remove),
                )
            });
        }

        diff.append(&mut vec![
            NetDiff::InternalServer(
                user.server.clone(),
                ServDiff::User(uid.clone(), ServAction::Remove),
            ),
            NetDiff::ExternalUser(uid, NetAction::Remove),
        ]);

        Ok(Outcome::State(diff))
    }
}
