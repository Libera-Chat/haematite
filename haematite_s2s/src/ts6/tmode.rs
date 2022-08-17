use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::network::Diff as NetDiff;

use super::util::mode::to_changes;
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars, ArgType};
use crate::util::DecodeHybrid;

//:420AAAAAB TMODE 1658071342 #test +bbb a!*@* b!*@* c!*@*
pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..)?;

    let channel_name = line.args[1].decode();
    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..])?;

    let mut diff = Vec::new();
    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        let arg = arg.map(DecodeHybrid::decode);
        let mode_diff = match change.arg_type {
            ArgType::None | ArgType::One => ChanDiff::Mode(
                change.mode,
                if change.remove {
                    ChanAction::Remove
                } else {
                    ChanAction::Add(arg)
                },
            ),
            ArgType::Many => ChanDiff::InternalModeList(
                change.mode,
                if change.remove {
                    ChanAction::Remove
                } else {
                    ChanAction::Add(arg.unwrap())
                },
            ),
        };

        diff.push(NetDiff::InternalChannel(channel_name.clone(), mode_diff))
    }

    Ok(Outcome::Empty)
}
