use haematite_models::hostmask::Hostmask;
use haematite_models::network::Network;
use haematite_models::topic::Topic;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use chrono::Utc;

//:420AAAABG TOPIC #test :hi
pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.source.as_ref().ok_or(Error::MissingSource)?;

    let user = network.get_user(uid)?;
    let hostmask = Hostmask {
        nick: user.nick.value.clone(),
        user: user.user.value.clone(),
        host: user.host.value.clone(),
    };

    let channel = network.get_channel_mut(&line.args[0])?;
    channel.topic = Some(Topic {
        text: line.args[1].decode(),
        since: Utc::now().naive_utc(),
        setter: hostmask,
    });

    Ok(Outcome::Empty)
}
