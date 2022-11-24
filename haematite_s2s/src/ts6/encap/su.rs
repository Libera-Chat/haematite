use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::user::Diff as UserDiff;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(3, 4, Self {})
    }
}

impl LineHandler for Handler {
    //:00A ENCAP * SU :420AAAAAB
    //:00A ENCAP * SU 420AAAAAB :jess
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        //Line::assert_arg_count(line, 3..4)?;

        let uid = line.args[2].decode();
        let account = line.args.get(3).map(DecodeHybrid::decode);

        Ok(Outcome::State(vec![NetDiff::InternalUser(
            uid,
            UserDiff::Account(account),
        )]))
    }
}
