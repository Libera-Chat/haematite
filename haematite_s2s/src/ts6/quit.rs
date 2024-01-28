use haematite_events::EventStore;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::{DecodeHybrid as _, TrueOr};

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;
    let server = user.server.clone();

    for channel_name in &user.channels {
        // this .ok_or() shouldn't be needed.
        // we've got a state desync if it is ever hit
        let channel = network
            .channels
            .get_mut(channel_name)
            .ok_or(Error::InvalidState)?;
        channel.users.remove(&uid).ok_or(Error::InvalidState)?;

        if channel.users.is_empty() && !channel.modes.contains_key(&'P') {
            network.channels.remove(channel_name);
        }
    }

    network.users.remove(&uid).ok_or(Error::InvalidState)?;
    network
        .servers
        .get_mut(&server)
        .ok_or(Error::InvalidState)?
        .users
        .remove(&uid)
        .true_or(Error::InvalidState)?;

    event_store.store(
        "user.disconnected",
        haematite_models::event::user::Disconnected { uid: &uid },
    )?;

    Ok(Outcome::Handled)
}
