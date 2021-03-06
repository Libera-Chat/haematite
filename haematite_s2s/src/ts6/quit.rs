use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

use super::util::del_user;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    let uid = line.source.as_ref().ok_or(Error::MissingSource)?;
    del_user(network, uid)?;

    Ok(Outcome::Empty)
}
