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
    Line::assert_arg_count(line, 3)?;

    let uid = line.source.as_ref().ok_or(Error::InvalidProtocol)?.decode();
    let certfp = line.args.get(3).map(DecodeHybrid::decode);

    if let Some(certfp) = &certfp {
        event_store.store(
            "user.certfp",
            haematite_models::event::user::HasCertfp { uid: &uid, certfp },
        )?;
    }

    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
    user.certfp = certfp;

    Ok(Outcome::Handled)
}
