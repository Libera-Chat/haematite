use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_chghost(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let args: &[Vec<u8>; 2] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing argument")?;

        let uid: &[u8; 9] = args[0].as_slice().try_into().map_err(|_| "invalid uid")?;
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).unwrap();
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;
        user.host = args[1].decode();

        Ok(Outcome::Empty)
    }
}
