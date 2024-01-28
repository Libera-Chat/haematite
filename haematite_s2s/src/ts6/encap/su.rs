use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

//:00A ENCAP * SU :420AAAAAB
//:00A ENCAP * SU 420AAAAAB :jess
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..4)?;

    let uid = line.args[2].decode();
    let account = line.args.get(3).map(DecodeHybrid::decode);

    event_store.store(
        "user.account",
        haematite_models::event::user::ChangeAccount {
            uid: &uid,
            account: &account,
        },
    )?;

    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
    user.account = account;

    Ok(Outcome::Handled)
}
