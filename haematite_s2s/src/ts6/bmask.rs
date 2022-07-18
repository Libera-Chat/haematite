use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::network::Diff as NetDiff;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let mode = line.args[2][0] as char;
    let masks_new = line.args[3].split(|c| c == &b' ');

    let mut diff = Vec::new();
    for mask in masks_new {
        diff.push(NetDiff::InternalChannel(
            channel_name.clone(),
            ChanDiff::ModeList(mode, mask.decode(), ChanAction::Add(None)),
        ));
    }

    Ok(Outcome::State(diff))
}
