use haematite_events::EventStore;
use haematite_models::irc::network::Network;
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::{DecodeHybrid as _, NoneOr};

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let sid = line.args[2].decode();
    let server = Server {
        id: sid.clone(),
        name: line.args[0].decode(),
        description: line.args[3].decode(),
        ..Server::default()
    };

    event_store.store(
        "server.connected",
        haematite_models::event::server::Connected {
            sid: &sid,
            name: &server.name,
            description: &server.description,
        },
    )?;

    network
        .servers
        .insert(sid, server)
        .none_or(Error::InvalidState)?;

    Ok(Outcome::Handled)
}
