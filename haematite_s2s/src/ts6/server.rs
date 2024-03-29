use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff};
use haematite_models::irc::server::Server;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

pub fn handle(ts6: &mut TS6Handler, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 3)?;

    let sid = ts6.uplink.take().ok_or(Error::InvalidState)?.decode();
    let server = Server::new(sid.clone(), line.args[0].decode(), line.args[2].decode());

    Ok(Outcome::State(vec![NetDiff::ExternalServer(
        sid,
        NetAction::Add(server),
    )]))
}
