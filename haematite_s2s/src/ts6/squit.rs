use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1..2)?;

    let sid = line.args[0].decode();
    let server = &network.servers[&sid];

    let mut diff = vec![NetDiff::ExternalServer(sid, NetAction::Remove)];

    for nick in &server.users {
        diff.push(NetDiff::ExternalUser(nick.clone(), NetAction::Remove));
    }

    Ok(Outcome::State(diff))
}
