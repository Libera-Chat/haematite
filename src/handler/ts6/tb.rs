use crate::handler::{Error, Outcome};
use crate::hostmask::Hostmask;
use crate::line::Line;
use crate::network::Network;
use crate::topic::Topic;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    //:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
    pub fn handle_tb(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() < 3 {
            return Err(Error::ExpectedArguments(3));
        }

        let channel = network
            .channels
            .get_mut(&line.args[0])
            .ok_or(Error::UnknownChannel)?;
        let since = line.args[1]
            .decode()
            .parse::<u32>()
            .map_err(|_| Error::BadArgument(1))?;
        let setter = Hostmask::try_from(line.args[2].decode().as_str())
            .map_err(|_| Error::BadArgument(2))?;

        let topic = Topic::new(line.args[3].decode(), since, setter);
        channel.topic = Some(topic);

        Ok(Outcome::Empty)
    }
}
