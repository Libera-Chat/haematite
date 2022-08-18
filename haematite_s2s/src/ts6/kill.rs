use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

//:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

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
