use crate::channel::Membership;
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;

use super::util::add_user_channel;
use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG JOIN 1657651885 #test +
    pub fn handle_join(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
        let channel = line.args.get(1).ok_or(Error::ExpectedArguments(1))?;

        add_user_channel(network, uid.clone(), channel, Membership::new())?;

        Ok(Outcome::Empty)
    }
}
