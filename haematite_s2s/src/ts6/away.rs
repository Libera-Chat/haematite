use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.users.get_mut(&uid).ok_or(Error::UnknownUser)?;
    let away = line.args.get(0).map(DecodeHybrid::decode);

    user.away = away;

    event_store.store(
        "user.away",
        haematite_models::event::user::Away {
            uid: &uid,
            away: &user.away,
        },
    )?;

    Ok(Outcome::Handled)
}
