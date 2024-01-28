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
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;

    let mode = line.args[2][0] as char;
    let masks_new = line.args[3].split(|c| c == &b' ');

    for mask in masks_new {
        let mask = mask.decode();
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
            .insert(mask, None);
    }

    Ok(Outcome::Handled)
}
