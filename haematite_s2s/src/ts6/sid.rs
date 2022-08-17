use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff};
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4)?;

    let sid = line.args[2].decode();
    let server = Server::new(sid.clone(), line.args[0].decode(), line.args[3].decode());

    Ok(Outcome::State(vec![NetDiff::ExternalServer(
        sid,
        NetAction::Add(server),
    )]))
}
