use haematite_events::EventStore;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let sid = line.args[0].decode();

    event_store.store(
        "server.disconnected",
        haematite_models::event::server::Disconnected { sid: &sid },
    )?;

    let server = network.servers.get(&sid).ok_or(StateError::UnknownServer)?;

    for uid in &server.users {
        event_store.store("user.lost", haematite_models::event::user::Lost { uid })?;

        network.users.remove(uid).ok_or(Error::InvalidState)?;
        let user = network.users.get(uid).ok_or(StateError::UnknownUser)?;

        for channel_name in &user.channels {
            let channel = network
                .channels
                .get_mut(channel_name)
                .ok_or(Error::InvalidState)?;

            channel.users.remove(uid).ok_or(Error::InvalidState)?;

            if channel.users.is_empty() && !channel.modes.contains_key(&'P') {
                network.channels.remove(channel_name);
            }
        }
    }

    Ok(Outcome::Handled)
}
