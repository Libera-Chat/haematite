use chrono::NaiveDateTime;
use haematite_models::hostmask::Hostmask;
use haematite_models::network::Network;
use haematite_models::topic::Topic;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    //:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
    pub fn handle_tb(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Line::assert_arg_count(line, 4..)?;

        let channel = network.get_channel_mut(&line.args[0])?;
        let since = line.args[1]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::InvalidArgument)?;

        let topic = Topic {
            text: line.args[3].decode(),
            since: NaiveDateTime::from_timestamp(since, 0),
            //TODO: handle missing setter
            setter: Hostmask::try_from(line.args[2].decode().as_str())
                .map_err(|_| Error::InvalidArgument)?,
        };
        channel.topic = Some(topic);

        Ok(Outcome::Empty)
    }
}
