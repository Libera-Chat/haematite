use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_quit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?.as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or("unknown sid")?;
        server.users.remove(uid).ok_or("unknown uid")?;

        Ok(Outcome::Empty)
    }
}
