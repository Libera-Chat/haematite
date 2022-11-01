use std::collections::HashSet;

use chrono::Utc;
use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff, ModeMetadata};
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::membership::{Action as MembAction, Diff as MembDiff};
use haematite_models::irc::network::{Diff as NetDiff, Network};

use super::util::mode::to_changes;
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars, ArgType};
use crate::util::DecodeHybrid;

//:420AAAAAB TMODE 1658071342 #test +bbb a!*@* b!*@* c!*@*
pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let setter = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

    let channel_name = line.args[1].decode();
    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..])?;

    let mut diff = Vec::new();
    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        let arg = arg.map(DecodeHybrid::decode);
        let mode_diff = if HashSet::from(['v', 'o']).contains(&change.mode) {
            ChanDiff::InternalUser(
                arg.ok_or(Error::MissingArgument)?,
                MembDiff::Status(
                    change.mode,
                    if change.remove {
                        MembAction::Remove
                    } else {
                        MembAction::Add
                    },
                ),
            )
        } else {
            match change.arg_type {
                ArgType::None | ArgType::One => ChanDiff::Mode(
                    change.mode,
                    if change.remove {
                        ChanAction::Remove
                    } else {
                        ChanAction::Add(arg)
                    },
                ),
                ArgType::Many => {
                    // this shouldn't possibly be None; `pair_args` should have
                    // already thrown this
                    let arg = arg.ok_or(Error::MissingArgument)?;
                    ChanDiff::ModeList(
                        change.mode,
                        arg,
                        if change.remove {
                            ChanAction::Remove
                        } else {
                            ChanAction::Add(Some(ModeMetadata {
                                since: Utc::now().naive_utc(),
                                setter: setter.hostmask(),
                            }))
                        },
                    )
                }
            }
        };

        diff.push(NetDiff::InternalChannel(channel_name.clone(), mode_diff));
    }

    Ok(Outcome::State(diff))
}
