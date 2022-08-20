use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};

use super::util::channel::{ForgetContext, Forgettable as _};
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

    let mut diff = vec![
        NetDiff::InternalServer(
            user.server.clone(),
            ServDiff::User(uid.clone(), ServAction::Remove),
        ),
        NetDiff::ExternalUser(uid, NetAction::Remove),
    ];

    for channel_name in &user.channels {
        // this .ok_or() shouldn't be needed.
        // we've got a state desync if it is ever hit
        let channel = network
            .channels
            .get(channel_name)
            .ok_or(StateError::UnknownChannel)?;
        if channel.is_forgettable(ForgetContext::Leave(1)) {
            diff.push(NetDiff::ExternalChannel(
                channel_name.clone(),
                NetAction::Remove,
            ));
        }
    }

    Ok(Outcome::State(diff))
}
