use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = &line.args[0];
    let user = network.get_user_mut(uid)?;

    for (mode, remove) in modes_from(&line.args[1].decode()) {
        if remove {
            user.modes.value.remove(&mode);
        } else {
            user.modes.value.insert(mode);
        }
    }

    if user.oper.value.is_some() && !user.modes.value.contains(&'o') {
        /* something (hopefully this mode change) caused this user to lose +o,
        so they're no longer opered */
        user.oper = None.into();
    }

    Ok(Outcome::Empty)
}
