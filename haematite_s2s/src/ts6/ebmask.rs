use chrono::NaiveDateTime;
use haematite_events::EventStore;
use haematite_models::irc::channel::ModeMetadata;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;

    let mode = line.args[2][0] as char;
    let parts = line.args[3].split(|c| c == &b' ').collect::<Vec<&[u8]>>();

    let chunks = parts.chunks_exact(3);
    if !chunks.remainder().is_empty() {
        return Err(Error::InvalidArgument);
    }

    for chunk in chunks {
        let mask = chunk[0].decode();
        let since = NaiveDateTime::from_timestamp(
            chunk[1]
                .decode()
                .parse::<i64>()
                .map_err(|_| Error::InvalidArgument)?,
            0,
        );
        let setter = chunk[2].decode();

        event_store.store(
            "channel.list_mode.add",
            haematite_models::event::channel::AddListMode {
                name: &channel_name,
                mode: &mode,
                mask: &mask,
            },
        )?;

        channel
            .mode_lists
            .get_mut(&mode)
            .ok_or(Error::InvalidState)?
            .insert(mask, Some(ModeMetadata { since, setter }));
    }

    Ok(Outcome::Handled)
}
