use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::{DecodeHybrid as _, TrueOr};

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.args[0].decode();
    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;

    for (mode, remove) in split_chars(&line.args[1].decode()) {
        if remove {
            event_store.store(
                "user.mode.remove",
                haematite_models::event::user::RemoveMode {
                    uid: &uid,
                    mode: &mode,
                },
            )?;

            if mode == 'o' {
                user.oper = None;
            }
            user.modes.remove(&mode).true_or(Error::InvalidState)?;
        } else {
            event_store.store(
                "user.mode.add",
                haematite_models::event::user::AddMode {
                    uid: &uid,
                    mode: &mode,
                },
            )?;

            user.modes.insert(mode).true_or(Error::InvalidState)?;
        }
    }

    Ok(Outcome::Handled)
}
