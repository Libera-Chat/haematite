use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

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
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let source = line
            .source
            .as_ref()
            .unwrap_or(&line.args[line.args.len() - 1])
            .decode();
        let me = network
            .servers
            .get(&network.me)
            .ok_or(StateError::UnknownServer)?;
        Ok(Outcome::Response(vec![format!(
            ":{} PONG {} {}",
            me.id, me.name, source
        )]))
    }
}
