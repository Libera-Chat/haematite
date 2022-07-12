use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::TrueOr as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_squit(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid = line.args.get(0).ok_or("missing argument")?;
        network.servers.remove(sid);

        for uid in network.server_users.remove(sid).ok_or("unknown sid")? {
            network.user_server.remove(&uid).ok_or("unknown uid")?;

            for channel_name in network
                .user_channels
                .remove(&uid)
                .ok_or("unknown uid")?
                .keys()
            {
                let channel = network
                    .channels
                    .get_mut(channel_name)
                    .ok_or("unknown channel")?;

                let channel_users = network
                    .channel_users
                    .get_mut(channel_name)
                    .ok_or("unknown channel")?;
                channel_users.remove(&uid).true_or("unknown uid")?;

                if channel_users.is_empty() && !channel.modes.contains_key(&'P') {
                    // unwrap shouldn't fail; we just read this channel name out
                    network.channels.remove(channel_name).unwrap();
                }
            }
        }

        Ok(Outcome::Empty)
    }
}
