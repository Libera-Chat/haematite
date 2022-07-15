use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

use super::TS6Handler;

pub fn handle(ts6: &mut TS6Handler, _network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    ts6.uplink = Some(line.args[3].clone());
    Ok(Outcome::Empty)
}
