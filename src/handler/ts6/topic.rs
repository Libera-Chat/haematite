use crate::handler::Outcome;
use crate::hostmask::Hostmask;
use crate::line::Line;
use crate::network::Network;
use crate::topic::Topic;
use crate::util::DecodeHybrid as _;

use chrono::Utc;

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG TOPIC #test :hi
    pub fn handle_topic(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let uid = line.source.as_ref().ok_or("missing source")?.as_slice();
        let sid = &uid[..3];

        let server = network.servers.get_mut(sid).ok_or("unknown sid")?;
        let user = server.users.get_mut(uid).ok_or("unknown uid")?;
        let channel = network
            .channels
            .get_mut(&line.args[0])
            .ok_or("unknown channel")?;

        let hostmask = Hostmask {
            nick: user.nickname.clone(),
            user: user.username.clone(),
            host: user.host.clone(),
        };

        channel.topic = Some(Topic {
            text: line.args[1].decode(),
            since: Utc::now().naive_utc(),
            setter: hostmask,
        });

        Ok(Outcome::Empty)
    }
}
