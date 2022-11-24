use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(2, 2, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        Ok(Outcome::State(vec![NetDiff::InternalUser(
            line.args[0].decode(),
            UserDiff::Host(line.args[1].decode()),
        )]))
    }
}
