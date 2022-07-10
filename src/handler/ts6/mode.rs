use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_mode(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let args: &[Vec<u8>; 2] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing arugment")?;

        let uid: &[u8; 9] = args[0].as_slice().try_into().map_err(|_| "invalid uid")?;
        let sid: &[u8; 3] = &uid[..3].try_into().unwrap();

        let server = network.servers.get_mut(sid).ok_or("unknown sid")?;
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;

        for (mode, remove) in modes_from(&args[1].decode()) {
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
