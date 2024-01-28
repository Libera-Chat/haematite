use haematite_events::EventStore;
use haematite_models::irc::network::Network;
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    // we should only get this command once, from our direct uplink, and we
    // should only get it after we've seen PASS but before we've seen any SID
    // which means the only server in `network.servers` is our uplink
    let sid = network
        .servers
        .keys()
        .next()
        .ok_or(Error::InvalidState)?
        .clone();

    let server = Server {
        id: sid.clone(),
        name: line.args[0].decode(),
        description: line.args[2].decode(),
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

    network.servers.insert(sid, server);

    Ok(Outcome::Handled)
}
