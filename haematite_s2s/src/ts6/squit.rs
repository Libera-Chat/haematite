use std::collections::HashMap;

use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};

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
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let sid = line.args[0].decode();
        let server = network.servers.get(&sid).ok_or(StateError::UnknownServer)?;

        let mut diff = vec![NetDiff::ExternalServer(sid, NetAction::Remove)];

        let mut channel_users = HashMap::new();
        for nick in &server.users {
            // this .ok_or() shouldn't be needed.
            // we've got a state desync if it is ever hit
            let user = network.users.get(nick).ok_or(StateError::UnknownUser)?;
            diff.push(NetDiff::ExternalUser(nick.clone(), NetAction::Remove));

            for channel_name in &user.channels {
                let count = channel_users.entry(channel_name).or_insert(0);
                *count += 1;
            }
        }

        for (channel_name, user_count) in channel_users {
            // this .ok_or() shouldn't be needed.
            // we've got a state desync if it is ever hit
            let channel = network
                .channels
                .get(channel_name)
                .ok_or(StateError::UnknownChannel)?;
            if channel.is_forgettable(ForgetContext::Leave(user_count)) {
                diff.push(NetDiff::ExternalChannel(
                    channel_name.to_string(),
                    NetAction::Remove,
                ));
            }
        }

        Ok(Outcome::State(diff))
    }
}
