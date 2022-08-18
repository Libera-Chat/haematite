use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

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
