use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_ping(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 1..)?;

        let source = line
            .source
            .as_ref()
            .unwrap_or(&line.args[line.args.len() - 1])
            .decode();
        let me = &network.servers[&network.me];
        Ok(Outcome::Response(vec![format!(
            ":{} PONG {} {}",
            me.sid, me.name, source
        )]))
    }
}
