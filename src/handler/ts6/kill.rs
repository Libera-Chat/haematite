use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::TrueOr as _;

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG KILL 111AAAABL :husky.vpn.lolnerd.net!user/jess!AkKA8fZrCB!jess (test reason)
    pub fn handle_kill(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = &line.args[0];

        network.users.remove(uid).ok_or("unknown uid")?;
        let sid = network.user_server.remove(uid).ok_or("unknown uid")?;
        network.servers.remove(&sid).ok_or("unknown sid")?;

        for channel_name in network
            .user_channels
            .remove(uid)
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

            channel_users.remove(uid).true_or("unknown uid")?;
            if channel_users.is_empty() && !channel.modes.contains_key(&'P') {
                // unwrap shouldn't fail, we just read it
                network.channels.remove(channel_name).unwrap();
            }
        }

        Ok(Outcome::Empty)
    }
}
