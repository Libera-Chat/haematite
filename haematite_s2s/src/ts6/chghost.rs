use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.args[0].decode();
    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
    user.host = line.args[1].decode();

    event_store.store(
        "user.host",
        haematite_models::event::user::ChangeHost {
            uid: &uid,
            host: &user.host,
        },
    )?;

    Ok(Outcome::Handled)
}
