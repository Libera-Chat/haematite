use haematite_models::irc::network::Network;

use super::util::mode::to_changes;
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::{pair_args, split_chars};
use crate::util::DecodeHybrid;

//:420AAAAAB TMODE 1658071342 #test +bbb a!*@* b!*@* c!*@*
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3..)?;

    let channel = network.get_channel_mut(&line.args[1].decode())?;
    let modes = to_changes(split_chars(&line.args[2].decode()));
    let mode_args = pair_args(&modes, &line.args[3..])?;

    for (change, arg) in modes.iter().zip(mode_args.iter()) {
        if change.remove {
            channel.modes.remove(&change.mode);
        } else {
            channel
                .modes
                .insert(change.mode, arg.map(DecodeHybrid::decode));
        }
    }

    Ok(Outcome::Empty)
}
