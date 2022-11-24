use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};

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
    //:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.args[0].decode();
        let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;
        Ok(Outcome::State(vec![
            NetDiff::InternalServer(
                user.server.clone(),
                ServDiff::User(uid.clone(), ServAction::Remove),
            ),
            NetDiff::ExternalUser(uid, NetAction::Remove),
        ]))
    }
}
