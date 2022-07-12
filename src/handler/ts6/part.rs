use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::util::del_user_channel;
use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG PART #test
    pub fn handle_part(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
        let channel_name = line.args.get(0).ok_or(Error::ExpectedArguments(1))?;
        del_user_channel(network, uid, channel_name)?;

        Ok(Outcome::Empty)
    }
}
