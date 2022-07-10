use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_away(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?.as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).unwrap();
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;
        user.away = line.args.get(0).map(DecodeHybrid::decode);

        Ok(Outcome::Empty)
    }
}
