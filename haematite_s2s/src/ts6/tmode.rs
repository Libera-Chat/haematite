use chrono::Utc;
use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff, ModeMetadata};
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::membership::{Action as MembAction, Diff as MembDiff};
use haematite_models::irc::network::{Diff as NetDiff, Network};

use super::util::mode::to_changes;
use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::mode::{split_chars, ArgType};
use crate::util::DecodeHybrid;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(3, usize::MAX, Self {})
    }
}

impl LineHandler for Handler {
    //:420AAAAAB TMODE 1658071342 #test +bbb a!*@* b!*@* c!*@*
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
        let setter = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

        let channel_name = line.args[1].decode();
        let modes = to_changes(split_chars(&line.args[2].decode()));
        let mut args = line.args[3..].iter();
        let args_total = args.len();

        let mut diff = Vec::new();
        for (i, change) in modes.iter().enumerate() {
            let mode_diff = match change.arg_type {
                ArgType::None => ChanDiff::Mode(
                    change.mode,
                    if change.remove {
                        ChanAction::Remove
                    } else {
                        ChanAction::Add(None)
                    },
                ),
                ArgType::One => {
                    let arg = args.next().map(DecodeHybrid::decode).ok_or_else(|| {
                        Error::InsufficientArguments {
                            expected: i,
                            actual: args_total,
                        }
                    })?;
                    ChanDiff::Mode(
                        change.mode,
                        if change.remove {
                            ChanAction::Remove
                        } else {
                            ChanAction::Add(Some(arg))
                        },
                    )
                }
                ArgType::Status => {
                    let arg = args.next().map(DecodeHybrid::decode).ok_or_else(|| {
                        Error::InsufficientArguments {
                            expected: i,
                            actual: args_total,
                        }
                    })?;

                    ChanDiff::InternalUser(
                        arg,
                        MembDiff::Status(
                            change.mode,
                            if change.remove {
                                MembAction::Remove
                            } else {
                                MembAction::Add
                            },
                        ),
                    )
                }
                ArgType::Many => {
                    let arg = args.next().map(DecodeHybrid::decode).ok_or_else(|| {
                        Error::InsufficientArguments {
                            expected: i,
                            actual: args_total,
                        }
                    })?;

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
            };

            diff.push(NetDiff::InternalChannel(channel_name.clone(), mode_diff));
        }

        Ok(Outcome::State(diff))
    }
}
