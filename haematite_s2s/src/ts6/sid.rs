use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::Server;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(4, 4, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let sid = line.args[2].decode();
        let server = Server::new(sid.clone(), line.args[0].decode(), line.args[3].decode());

        Ok(Outcome::State(vec![NetDiff::ExternalServer(
            sid,
            NetAction::Add(server),
        )]))
    }
}
