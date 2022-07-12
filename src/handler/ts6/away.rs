use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_away(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
        let user = network.get_user_mut(uid)?;
        user.away = line.args.get(0).map(DecodeHybrid::decode);

        Ok(Outcome::Empty)
    }
}
