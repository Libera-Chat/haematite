use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.args[0].decode();
    let user = network.get_user_mut(&uid)?;

    let mut deopered = false;

    for (mode, remove) in split_chars(&line.args[1].decode()) {
        if remove {
            deopered |= mode == 'o';
            user.modes.remove(&mode);
        } else {
            user.modes.insert(mode);
        }
    }

    if deopered {
        // they've lost umode +o, thus are no longer an oper
        user.oper = None;
    }

    Ok(Outcome::Empty)
}
