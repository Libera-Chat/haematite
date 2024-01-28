use chrono::NaiveDateTime;
use haematite_events::EventStore;
use haematite_models::irc::hostmask::Hostmask;
use haematite_models::irc::network::Network;
use haematite_models::irc::topic::{Setter, Topic};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

//:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4..)?;

    let channel_name = line.args[0].decode();
    let text = line.args[3].decode();
    let since = NaiveDateTime::from_timestamp(
        line.args[1]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::InvalidArgument)?,
        0,
    );
    let setter = line.args[2].decode();

    event_store.store(
        "channel.topic.burst",
        haematite_models::event::channel::TopicBurst {
            name: &channel_name,
            text: &text,
            since: &since,
            setter: &setter,
        },
    )?;

    let setter = Hostmask::try_from(setter.as_str())
        .map_or_else(|_| Setter::Nickname(setter), Setter::Hostmask);

    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;
    channel.topic = Some(Topic {
        text,
        since,
        //TODO: handle missing setter
        setter,
    });

    Ok(Outcome::Handled)
}
