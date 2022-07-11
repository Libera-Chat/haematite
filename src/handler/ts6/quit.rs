use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_quit(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or(Error::UnknownServer)?;
        server.users.remove(uid).ok_or(Error::UnknownUser)?;

        Ok(Outcome::Empty)
    }
}
