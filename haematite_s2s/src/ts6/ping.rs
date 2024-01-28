use haematite_events::EventStore;
use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle<E: EventStore>(
    _event_store: &mut E,
    network: &Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let source = line
        .source
        .as_ref()
        .unwrap_or(&line.args[line.args.len() - 1])
        .decode();

    Ok(Outcome::Responses(vec![format!(
        ":{} PONG {} {source}",
        network.me.id, network.me.name,
    )]))
}
