use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::util::del_user;
use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
    pub fn handle_kill(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = &line.args[0];
        del_user(network, uid)?;

        Ok(Outcome::Empty)
    }
}
