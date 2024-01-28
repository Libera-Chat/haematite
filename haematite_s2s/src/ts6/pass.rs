use haematite_events::EventStore;
use haematite_models::irc::network::Network;
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::NoneOr;
use crate::DecodeHybrid;

pub fn handle<E: EventStore>(
    _event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let sid = line.args[3].decode();
    network
        .servers
        .insert(sid, Server::default())
        .none_or(Error::InvalidState)?;

    Ok(Outcome::Handled)
}
