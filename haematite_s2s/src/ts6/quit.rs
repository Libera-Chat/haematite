use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

    Ok(Outcome::State(vec![
        NetDiff::InternalServer(
            user.server.clone(),
            ServDiff::User(uid.clone(), ServAction::Remove),
        ),
        NetDiff::ExternalUser(uid, NetAction::Remove),
    ]))
}
