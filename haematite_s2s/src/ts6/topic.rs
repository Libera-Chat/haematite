use haematite_events::EventStore;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::hostmask::Hostmask;
use haematite_models::irc::network::Network;
use haematite_models::irc::topic::{Setter, Topic};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use chrono::Utc;

//:420AAAABG TOPIC #test :hi
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let channel_name = line.args[0].decode();
    let text = line.args[1].decode();

    let topic = if text.is_empty() {
        event_store.store(
            "channel.topic.remove",
            haematite_models::event::channel::RemoveTopic {
                name: &channel_name,
                uid: &uid,
            },
        )?;
        None
    } else {
        let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;
        let hostmask = Hostmask {
            nick: user.nick.clone(),
            user: user.user.clone(),
            host: user.host.clone(),
        };

        event_store.store(
            "channel.topic.change",
            haematite_models::event::channel::ChangeTopic {
                name: &channel_name,
                uid: &uid,
                text: &text,
            },
        )?;

        Some(Topic {
            text,
            since: Utc::now().naive_utc(),
            setter: Setter::Hostmask(hostmask),
        })
    };

    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;
    channel.topic = topic;

    Ok(Outcome::Handled)
}
