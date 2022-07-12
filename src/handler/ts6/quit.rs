use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::TrueOr as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_quit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?;
        network.users.remove(uid).ok_or("unknown uid")?;
        network.user_server.remove(uid).ok_or("unknown uid")?;

        for channel in network
            .user_channels
            .remove(uid)
            .ok_or("unknown uid")?
            .keys()
        {
            network
                .channel_users
                .get_mut(channel)
                .ok_or("unknown channel")?
                .remove(uid)
                .true_or("unknown uid")?;
        }

        Ok(Outcome::Empty)
    }
}
