use std::collections::{HashMap, HashSet};

use haematite_models::irc::channel::Channel;
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars};
use crate::util::DecodeHybrid;

use super::util::mode::to_changes;

//:00A SJOIN 1658071435 #services +nst :@00AAAAAAB
pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let uids = line.args[line.args.len() - 1]
        .split(|c| c == &b' ')
        // we may get an empty last param
        .filter(|a| !a.is_empty())
        .collect::<Vec<&[u8]>>();

    let mut diff = Vec::new();

    let mut new_channel = Channel::new(HashSet::from_iter(vec!['I', 'b', 'e', 'q']));

    if let Some(channel) = network.channels.get(&channel_name) {
        for nick in channel.users.keys() {
            new_channel.users.insert(nick.clone(), Membership::new());
        }
    }

    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..line.args.len() - 1])?;
    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        new_channel
            .modes
            .insert(change.mode, arg.map(DecodeHybrid::decode));
    }

    for uid in uids {
        //TODO: precompile
        let statuses = HashMap::from([('+', 'v'), ('@', 'o')]);

        let mut uid = uid.decode();

        let mut membership = Membership::new();

        while let Some(char) = uid.chars().next() {
            if let Some(mode) = statuses.get(&char) {
                membership.status.push(*mode);
                uid.remove(0);
            } else {
                break;
            }
        }

        if uid.is_empty() {
            return Err(Error::InvalidArgument);
        }

        diff.push(NetDiff::InternalUser(
            uid.clone(),
            UserDiff::Channel(channel_name.clone(), UserAction::Add),
        ));
        new_channel.users.insert(uid, membership);
    }

    diff.insert(
        0,
        NetDiff::ExternalChannel(channel_name, NetAction::Add(new_channel)),
    );

    Ok(Outcome::State(diff))
}
