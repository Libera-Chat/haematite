use chrono::NaiveDateTime;
use haematite_models::channel::ModeMetadata;
use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel = network.get_channel_mut(&line.args[1])?;
    let mode = line.args[2][0] as char;

    let masks = channel
        .mode_lists
        .entry(mode)
        .or_insert_with(Default::default);

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

        masks.insert(mask, Some(ModeMetadata { since, setter }));
    }

    Ok(Outcome::Empty)
}
