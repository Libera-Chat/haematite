use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::util::del_server;
use super::TS6Handler;

impl TS6Handler {
    pub fn handle_squit(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let sid = line.args.get(0).ok_or(Error::ExpectedArguments(1))?;
        del_server(network, sid)?;

        Ok(Outcome::Empty)
    }
}
