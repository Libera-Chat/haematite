use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::TrueOr as _;

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG PART #test
    pub fn handle_part(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?;
        let channel_name = line.args.get(0).ok_or("missing channel")?;
        let channel = network
            .channels
            .get(channel_name)
            .ok_or("unknown channel")?;

        network
            .user_channels
            .get_mut(uid)
            .ok_or("unknown uid")?
            .remove(channel_name)
            .ok_or("unknown channel")?;

        let channel_users = network
            .channel_users
            .get_mut(channel_name)
            .ok_or("unknown channel")?;
        channel_users.remove(uid).true_or("unknown uid")?;

        if channel_users.is_empty() && !channel.modes.contains_key(&'P') {
            network.channels.remove(channel_name);
            network.channel_users.remove(channel_name);
        }

        Ok(Outcome::Empty)
    }
}
