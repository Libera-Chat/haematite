use std::collections::HashSet;

use crate::channel::{Channel, Membership};
use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::{DecodeHybrid, NoneOr as _, TrueOr as _};

use super::{parse_mode_args, TS6Handler};

impl TS6Handler {
    pub fn handle_sjoin(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() < 4 {
            return Err("unexpected argument count");
        }

        let channel_name = &line.args[1];
        let uids = line.args[line.args.len() - 1]
            .split(|c| c == &b' ')
            .collect::<Vec<&[u8]>>();

        let mut channel = Channel::new();

        let modes = modes_from(&line.args[2].decode());
        let mode_args = line.args[3..line.args.len() - 1].iter();
        for (mode, _, arg) in parse_mode_args(modes, mode_args) {
            channel.modes.insert(mode, arg.map(DecodeHybrid::decode));
        }

        network.channels.insert(channel_name.clone(), channel);
        network
            .channel_users
            .insert(channel_name.clone(), HashSet::default())
            .none_or("overwritten channel")?;

        // unwrap shouldn't fail; we just added it
        let channel_users = network.channel_users.get_mut(channel_name).unwrap();
        for uid in uids {
            //TODO: precompile
            let statuses = HashSet::from(['+', '@']);

            let mut uid = uid;

            let mut membership = Membership::new();
            while !uid.is_empty() && statuses.contains(&(uid[0] as char)) {
                membership
                    .status
                    .insert(uid[0] as char)
                    .true_or("overwritten status")?;
                uid = &uid[1..];
            }
            if uid.is_empty() {
                return Err("empty uid");
            }

            channel_users
                .insert(uid.to_vec())
                .true_or("overwritten uid")?;
            network
                .user_channels
                .get_mut(uid)
                .ok_or("unknown uid")?
                .insert(channel_name.clone(), membership)
                .none_or("overwritten channel")?;
        }

        Ok(Outcome::Empty)
    }
}
