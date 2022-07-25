use std::collections::HashSet;

use haematite_models::channel::{Channel, Membership};
use haematite_models::network::{Error as StateError, Network};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars};
use crate::util::DecodeHybrid;

use super::util::mode::to_changes;
use super::util::state::{add_channel, add_user_channel};

//:00A SJOIN 1658071435 #services +nst :@00AAAAAAB
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = line.args[1].decode();
    let uids = line.args[line.args.len() - 1]
        .split(|c| c == &b' ')
        // we may get an empty last param
        .filter(|a| !a.is_empty())
        .collect::<Vec<&[u8]>>();

    let channel_new = Channel::new();

    let channel = match add_channel(network, channel_name.clone(), channel_new) {
        Err(StateError::OverwrittenChannel) => {
            // SJOIN caused by a netjoin elsewhere that's overwriting this channel
            let channel = network.get_channel_mut(&channel_name)?;
            channel.modes.clear();
            channel.mode_lists.clear();
            channel
        }
        Err(_) => {
            return Err(Error::InvalidState);
        }
        Ok(_) => network.get_channel_mut(&channel_name)?,
    };

    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..line.args.len() - 1])?;
    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        channel
            .modes
            .insert(change.mode, arg.map(DecodeHybrid::decode));
    }

    for uid in uids {
        //TODO: precompile
        let statuses = HashSet::from(['+', '@']);

        let mut uid = uid.decode();

        let mut membership = Membership::new();

        while let Some(char) = uid.chars().next() {
            if !statuses.contains(&char) {
                break;
            }

            membership.status.insert(char);
            uid.remove(0);
        }

        if uid.is_empty() {
            return Err(Error::InvalidArgument);
        }

        add_user_channel(network, uid, &channel_name, membership)?;
    }

    Ok(Outcome::Empty)
}
