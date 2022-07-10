use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_quit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let source = line.source.as_ref().ok_or("missing source")?;
        let uid: &[u8; 9] = source.as_slice().try_into().map_err(|_| "invalid uid")?;
        // this is smaller than `uid`, so should never panic
        let sid: &[u8; 3] = uid[..3].try_into().unwrap();

        let server = network.servers.get_mut(sid).ok_or("unknown sid")?;
        server.users.remove(uid).ok_or("unknown uid")?;

        Ok(Outcome::Empty)
    }
}
