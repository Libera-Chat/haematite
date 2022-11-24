use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(1, 2, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
        let oper = Some(line.args[0].decode());

        Ok(Outcome::State(vec![NetDiff::InternalUser(
            uid,
            UserDiff::Oper(oper),
        )]))
    }
}
