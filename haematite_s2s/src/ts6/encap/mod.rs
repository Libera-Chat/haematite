mod certfp;
mod identified;
mod su;

use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2..)?;

    match line.args[1].as_slice() {
        b"CERTFP" => certfp::handle(event_store, network, line),
        b"SU" => su::handle(event_store, network, line),
        b"IDENTIFIED" => identified::handle(event_store, network, line),
        _ => Ok(Outcome::Unhandled),
    }
}
