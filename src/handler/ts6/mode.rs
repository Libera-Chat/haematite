use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_mode(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() != 2 {
            return Err("unexpected argument count");
        }

        let uid = line.args[0].as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or("unknown sid")?;
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;

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
