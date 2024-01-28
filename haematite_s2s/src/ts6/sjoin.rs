use std::collections::HashMap;

use haematite_events::EventStore;
use haematite_models::irc::channel::Channel;
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars};
use crate::util::DecodeHybrid;

use super::util::mode::to_changes;

//:00A SJOIN 1658071435 #services +nstk password :@00AAAAAAB
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4..)?;

    let timestamp: u64 = line.args[0].decode().parse().unwrap();
    let channel_name = line.args[1].decode();
    let uids = line.args[line.args.len() - 1]
        .split(|c| c == &b' ')
        // we may get an empty last param
        .filter(|a| !a.is_empty())
        .collect::<Vec<&[u8]>>();

    let channel = network
        .channels
        .entry(channel_name.clone())
        .or_insert_with(|| Channel {
            mode_lists: HashMap::from([
                ('I', HashMap::new()),
                ('b', HashMap::new()),
                ('e', HashMap::new()),
                ('q', HashMap::new()),
            ]),
            timestamp,
            ..Channel::default()
        });

    if timestamp < channel.timestamp {
        let mut new_channel = Channel {
            mode_lists: HashMap::from([
                ('I', HashMap::new()),
                ('b', HashMap::new()),
                ('e', HashMap::new()),
                ('q', HashMap::new()),
            ]),
            timestamp,
            ..Channel::default()
        };
        let users = channel
            .users
            .drain()
            .map(|(u, _)| (u, Membership::default()));
        new_channel.users = users.collect();
        std::mem::swap(channel, &mut new_channel);
    }

    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..line.args.len() - 1])?;
    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        channel
            .modes
            .insert(change.mode, arg.map(DecodeHybrid::decode));
    }

    let mut new_uids = Vec::new();
    let statuses = HashMap::from([(b'+', 'v'), (b'@', 'o')]);

    for mut uid in uids {
        let mut membership = Membership::default();

        for char in uid {
            if let Some(mode) = statuses.get(char) {
                membership.status.insert(*mode);
                uid = &uid[1..];
            } else {
                break;
            }
        }

        if uid.is_empty() {
            return Err(Error::InvalidArgument);
        }
        let uid = uid.decode();

        let user = network.users.get_mut(&uid).ok_or(Error::InvalidState)?;
        user.channels.insert(channel_name.clone());

        new_uids.push(uid.clone());
        channel.users.insert(uid, membership);
    }

    event_store.store(
        "channel.burst",
        haematite_models::event::channel::Burst {
            name: &channel_name,
            new_uids: &new_uids,
        },
    )?;

    Ok(Outcome::Handled)
}
