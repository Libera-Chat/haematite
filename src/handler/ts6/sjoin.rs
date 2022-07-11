use crate::channel::Channel;
use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::DecodeHybrid;

use super::{parse_mode_args, TS6Handler};

impl TS6Handler {
    pub fn handle_sjoin(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() < 4 {
            return Err("unexpected argument count");
        }

        let name = line.args[1].clone();
        let _users = line.args[line.args.len() - 1].decode().split(' ');

        let mut channel = Channel::new();

        let modes = modes_from(&line.args[2].decode());
        let mode_args = line.args[3..line.args.len() - 1].iter();
        for (mode, _, arg) in parse_mode_args(modes, mode_args) {
            channel.modes.insert(mode, arg.map(DecodeHybrid::decode));
        }

        network.channels.insert(name, channel);

        Ok(Outcome::Empty)
    }
}
