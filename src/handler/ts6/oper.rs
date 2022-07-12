use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_oper(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?.as_slice();
        let user = network.users.get_mut(uid).ok_or("unknown uid")?;
        user.oper = Some(line.args[0].decode());

        Ok(Outcome::Empty)
    }
}
