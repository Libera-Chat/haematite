use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_mode(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() != 2 {
            return Err(Error::ExpectedArguments(2));
        }

        let uid = &line.args[0];
        let user = network.get_user_mut(uid)?;

        for (mode, remove) in modes_from(&line.args[1].decode()) {
            if remove {
                user.modes.remove(&mode);
            } else {
                user.modes.insert(mode);
            }
        }

        if user.oper.is_some() && !user.modes.contains(&'o') {
            /* something (hopefully this mode change) caused this user to lose +o,
            so they're no longer opered */
            user.oper = None;
        }

        Ok(Outcome::Empty)
    }
}
