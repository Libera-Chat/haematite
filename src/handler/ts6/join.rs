use crate::channel::Membership;
use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::{NoneOr as _, TrueOr as _};

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG JOIN 1657651885 #test +
    pub fn handle_join(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?;
        let channel = line.args.get(1).ok_or("missing channel")?;

        let membership = Membership::new();

        network
            .user_channels
            .get_mut(uid)
            .ok_or("unknown uid")?
            .insert(channel.clone(), membership)
            .none_or("overwritten channel")?;
        network
            .channel_users
            .get_mut(channel)
            .ok_or("unknown channel")?
            .insert(uid.clone())
            .true_or("overwritten uid")?;

        Ok(Outcome::Empty)
    }
}
