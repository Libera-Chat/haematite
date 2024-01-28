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
    Line::assert_arg_count(line, 1..2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let oper = Some(line.args[0].decode());

    event_store.store(
        "user.oper",
        haematite_models::event::user::ChangeOper {
            uid: &uid,
            oper: &oper,
        },
    )?;

    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
    user.oper = oper;

    Ok(Outcome::Handled)
}
