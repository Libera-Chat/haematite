use std::collections::HashSet;

use haematite_models::channel::{Channel, Membership};
use haematite_models::network::{Error as StateError, Network};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::util::DecodeHybrid;

use super::parse_mode_args;
use super::util::{add_channel, add_user_channel};

//:00A SJOIN 1658071435 #services +nst :@00AAAAAAB
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let channel_name = &line.args[1];
    let uids = line.args[line.args.len() - 1]
        .split(|c| c == &b' ')
        .collect::<Vec<&[u8]>>();

    let channel_new = Channel::new();

    let channel = match add_channel(network, channel_name.clone(), channel_new) {
        Err(StateError::OverwrittenChannel) => {
            // SJOIN caused by a netjoin elsewhere that's overwriting this channel
            let channel = network.get_channel_mut(channel_name)?;
            channel.modes.clear();
            channel.mode_lists.clear();
            channel
        }
        Err(_) => {
            return Err(Error::InvalidState);
        }
        Ok(_) => network.get_channel_mut(channel_name)?,
    };

    let modes = modes_from(&line.args[2].decode());
    let mode_args = line.args[3..line.args.len() - 1].iter();
    for (mode, _, arg) in parse_mode_args(modes, mode_args) {
        channel.modes.insert(mode, arg.map(DecodeHybrid::decode));
    }

    for uid in uids {
        //TODO: precompile
        let statuses = HashSet::from(['+', '@']);

        let mut uid = uid;

        let mut membership = Membership::new();
        while !uid.is_empty() && statuses.contains(&(uid[0] as char)) {
            membership.status.insert(uid[0] as char);
            uid = &uid[1..];
        }
        if uid.is_empty() {
            return Err(Error::InvalidArgument);
        }

        add_user_channel(network, uid.to_vec(), channel_name, membership)?;
    }

    Ok(Outcome::Empty)
}
