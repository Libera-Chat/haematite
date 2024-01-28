use haematite_events::EventStore;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::{DecodeHybrid as _, TrueOr};

//:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.args[0].decode();

    event_store.store(
        "user.disconnected",
        haematite_models::event::user::Disconnected { uid: &uid },
    )?;

    let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

    network
        .servers
        .get_mut(&user.server)
        .ok_or(Error::InvalidState)?
        .users
        .remove(&uid)
        .true_or(Error::InvalidState)?;
    network.users.remove(&uid).ok_or(Error::InvalidState)?;

    Ok(Outcome::Handled)
}
