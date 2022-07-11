use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_chghost(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() != 2 {
            return Err("unexpected argument count");
        }

        let uid = line.args[0].as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).unwrap();
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;
        user.host = line.args[1].decode();

        Ok(Outcome::Empty)
    }
}
