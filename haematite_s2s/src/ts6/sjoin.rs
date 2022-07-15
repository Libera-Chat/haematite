use std::collections::HashSet;

use haematite_models::channel::{Channel, Membership};
use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::util::DecodeHybrid;

use super::util::{add_channel, add_user_channel};
use super::{parse_mode_args, TS6Handler};

impl TS6Handler {
    pub fn handle_sjoin(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Line::assert_arg_count(line, 4)?;

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

        add_channel(network, channel_name.clone(), channel)?;

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
}
