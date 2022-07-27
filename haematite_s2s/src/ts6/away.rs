use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let user = network.get_user_mut(&uid)?;
    user.away = line.args.get(0).map(DecodeHybrid::decode);

    Ok(Outcome::Empty)
}
