use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_chghost(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() != 2 {
            return Err(Error::ExpectedArguments(2));
        }

        let uid = line.args[0].as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).unwrap();
        let user = server.users.get_mut(uid).ok_or(Error::UnknownUser)?;
        user.host = line.args[1].decode();

        Ok(Outcome::Empty)
    }
}
