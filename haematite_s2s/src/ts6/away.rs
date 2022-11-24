use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(0, 1, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
        let away = line.args.get(0).map(DecodeHybrid::decode);

        Ok(Outcome::State(vec![NetDiff::InternalUser(
            uid,
            UserDiff::Away(away),
        )]))
    }
}
