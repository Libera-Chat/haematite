use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_oper(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or(Error::UnknownServer)?;
        let user = server.users.get_mut(uid).ok_or(Error::UnknownUser)?;
        user.oper = Some(line.args[0].decode());

        Ok(Outcome::Empty)
    }
}
