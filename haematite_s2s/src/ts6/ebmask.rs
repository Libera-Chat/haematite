use chrono::NaiveDateTime;
use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff, ModeMetadata};
use haematite_models::irc::network::Diff as NetDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let mode = line.args[2][0] as char;
    let parts = line.args[3].split(|c| c == &b' ').collect::<Vec<&[u8]>>();

    let chunks = parts.chunks_exact(3);
    if !chunks.remainder().is_empty() {
        return Err(Error::InvalidArgument);
    }

    let mut diff = Vec::new();
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

        diff.push(NetDiff::InternalChannel(
            channel_name.clone(),
            ChanDiff::ModeList(
                mode,
                mask,
                ChanAction::Add(Some(ModeMetadata { since, setter })),
            ),
        ));
    }

    Ok(Outcome::State(diff))
}
