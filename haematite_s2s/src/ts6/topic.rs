use haematite_models::irc::channel::Diff as ChanDiff;
use haematite_models::irc::hostmask::Hostmask;
use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::topic::{Setter, Topic};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use chrono::Utc;

//:420AAAABG TOPIC #test :hi
pub fn handle(network: &Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let channel_name = line.args[0].decode();
    let text = line.args[1].decode();

    let topic = if text.is_empty() {
        None
    } else {
        let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();

        let user = network.get_user(&uid)?;
        let hostmask = Hostmask {
            nick: user.nick.clone(),
            user: user.user.clone(),
            host: user.host.clone(),
        };

        Some(Topic {
            text: text,
            since: Utc::now().naive_utc(),
            setter: Setter::Hostmask(hostmask),
        })
    };

    Ok(Outcome::State(vec![NetDiff::InternalChannel(
        channel_name,
        ChanDiff::Topic(topic),
    )]))
}
