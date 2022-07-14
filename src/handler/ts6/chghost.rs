use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_chghost(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 2)?;

        let uid = &line.args[0];
        let user = network.get_user_mut(uid)?;
        user.host = line.args[1].decode();

        Ok(Outcome::Empty)
    }
}
