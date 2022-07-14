use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::util::del_user;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_quit(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
        del_user(network, uid)?;

        Ok(Outcome::Empty)
    }
}
