use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_oper(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 1..)?;

        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
        let user = network.get_user_mut(uid)?;
        user.oper = Some(line.args[0].decode());

        Ok(Outcome::Empty)
    }
}
