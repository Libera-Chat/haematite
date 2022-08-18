use chrono::NaiveDateTime;
use haematite_models::irc::channel::Diff as ChanDiff;
use haematite_models::irc::hostmask::Hostmask;
use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::topic::{Setter, Topic};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

//:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 4..)?;

    let channel_name = line.args[0].decode();
    let since = line.args[1]
        .decode()
        .parse::<i64>()
        .map_err(|_| Error::InvalidArgument)?;

    let setter = line.args[2].decode();
    let setter = match Hostmask::try_from(setter.as_str()) {
        Ok(hostmask) => Setter::Hostmask(hostmask),
        Err(_) => Setter::Nickname(setter),
    };

    let topic = Topic {
        text: line.args[3].decode(),
        since: NaiveDateTime::from_timestamp(since, 0),
        //TODO: handle missing setter
        setter,
    };

    Ok(Outcome::State(vec![NetDiff::InternalChannel(
        channel_name,
        ChanDiff::Topic(Some(topic)),
    )]))
}
