use haematite_models::hostmask::Hostmask;
use haematite_models::network::Network;
use haematite_models::topic::Topic;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use chrono::Utc;

use super::TS6Handler;

impl TS6Handler {
    //:420AAAABG TOPIC #test :hi
    pub fn handle_topic(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Line::assert_arg_count(line, 2)?;

        let uid = line.source.as_ref().ok_or(Error::MissingSource)?;

        let user = network.get_user(uid)?;
        let hostmask = Hostmask {
            nick: user.nickname.clone(),
            user: user.username.clone(),
            host: user.host.clone(),
        };

        let channel = network.get_channel_mut(&line.args[0])?;
        channel.topic = Some(Topic {
            text: line.args[1].decode(),
            since: Utc::now().naive_utc(),
            setter: hostmask,
        });

        Ok(Outcome::Empty)
    }
}
