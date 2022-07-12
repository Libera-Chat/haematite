use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::DecodeHybrid;

use super::{parse_mode_args, TS6Handler};

impl TS6Handler {
    pub fn handle_tmode(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() < 3 {
            return Err(Error::ExpectedArguments(3));
        }

        let channel = network.get_channel_mut(&line.args[1])?;
        let modes = modes_from(&line.args[2].decode());
        let mode_args = line.args[3..].iter();

        for (mode, remove, arg) in parse_mode_args(modes, mode_args) {
            if remove {
                channel.modes.remove(&mode);
            } else {
                channel.modes.insert(mode, arg.map(DecodeHybrid::decode));
            }
        }

        Ok(Outcome::Empty)
    }
}
