use haematite_events::EventStore;
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::{DecodeHybrid as _, FalseOr};

//:420AAAABG JOIN 1657651885 #test +
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let channel_name = line.args[1].decode();

    let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;

    event_store.store(
        "user.join",
        haematite_models::event::user::Join {
            uid: &uid,
            channel: &channel_name,
        },
    )?;

    user.channels
        .insert(channel_name)
        .false_or(Error::InvalidState)?;

    channel.users.insert(uid, Membership::default());

    Ok(Outcome::Handled)
}
