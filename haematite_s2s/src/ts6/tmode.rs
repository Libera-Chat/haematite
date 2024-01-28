use chrono::Utc;
use haematite_events::EventStore;
use haematite_models::irc::channel::ModeMetadata;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use super::util::mode::to_changes;
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars, ArgType};
use crate::util::DecodeHybrid;

//:420AAAAAB TMODE 1658071342 #test +bbb a!*@* b!*@* c!*@*
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let setter = network.users.get(&uid).ok_or(StateError::UnknownUser)?;

    let channel_name = line.args[1].decode();
    let channel = network
        .channels
        .get_mut(&channel_name)
        .ok_or(Error::InvalidState)?;
    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..])?;

    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        let arg = arg.map(DecodeHybrid::decode);
        match change.arg_type {
            ArgType::Status => {
                let membership = channel
                    .users
                    .get_mut(&arg.ok_or(Error::MissingArgument)?)
                    .ok_or(Error::InvalidState)?;
                if change.remove {
                    membership.status.remove(&change.mode);
                } else {
                    membership.status.insert(change.mode);
                }
            }
            ArgType::None | ArgType::One => {
                if change.remove {
                    event_store.store(
                        "channel.mode.remove",
                        haematite_models::event::channel::RemoveMode {
                            channel: &channel_name,
                            mask: &arg,
                        },
                    )?;
                    channel.modes.remove(&change.mode);
                } else {
                    event_store.store(
                        "channel.mode.add",
                        haematite_models::event::channel::AddMode {
                            channel: &channel_name,
                            mask: &arg,
                        },
                    )?;
                    channel.modes.insert(change.mode, arg);
                }
            }
            ArgType::Many => {
                // this shouldn't possibly be None; `pair_args` should have
                // already thrown this
                let arg = arg.ok_or(Error::MissingArgument)?;
                let mode_list = channel
                    .mode_lists
                    .get_mut(&change.mode)
                    .ok_or(Error::InvalidState)?;
                if change.remove {
                    event_store.store(
                        "channel.list_mode.remove",
                        haematite_models::event::channel::RemoveListMode {
                            name: &channel_name,
                            mode: &change.mode,
                            mask: &arg,
                        },
                    )?;
                    mode_list.remove(&arg);
                } else {
                    event_store.store(
                        "channel.list_mode.add",
                        haematite_models::event::channel::AddListMode {
                            name: &channel_name,
                            mode: &change.mode,
                            mask: &arg,
                        },
                    )?;
                    mode_list.insert(
                        arg,
                        Some(ModeMetadata {
                            since: Utc::now().naive_utc(),
                            setter: setter.hostmask(),
                        }),
                    );
                }
            }
        }
    }

    Ok(Outcome::Handled)
}
