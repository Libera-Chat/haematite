use haematite_events::EventStore;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

//:420AAAABG PART #test
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let channel_name = line.args[0].decode();

    event_store.store(
        "user.part",
        haematite_models::event::user::Part {
            uid: &uid,
            channel: &channel_name,
        },
    )?;

    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(StateError::UnknownChannel)?;

    channel.users.remove(&uid).ok_or(Error::InvalidState)?;
    if channel.users.is_empty() && !channel.modes.contains_key(&'P') {
        network.channels.remove(&channel_name);
    }

    Ok(Outcome::Handled)
}
