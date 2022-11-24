use std::cell::RefCell;
use std::rc::Rc;

use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::server::Server;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {
    uplink: Rc<RefCell<Option<Vec<u8>>>>,
}

impl Handler {
    pub fn resolver(uplink: Rc<RefCell<Option<Vec<u8>>>>) -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(3, 3, Self { uplink })
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let sid = self
            .uplink
            .replace(None)
            .ok_or(Error::InvalidState)?
            .decode();
        let server = Server::new(sid.clone(), line.args[0].decode(), line.args[2].decode());

        Ok(Outcome::State(vec![NetDiff::ExternalServer(
            sid,
            NetAction::Add(server),
        )]))
    }
}
