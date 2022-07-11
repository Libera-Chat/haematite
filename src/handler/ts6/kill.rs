use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
    pub fn handle_kill(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.args[0].as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or("unknown server")?;
        server.users.remove(uid);

        Ok(Outcome::Empty)
    }
}
