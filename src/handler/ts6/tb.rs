use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::topic::Topic;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    //:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
    pub fn handle_tb(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let channel = network
            .channels
            .get_mut(&line.args[0])
            .ok_or("unknown channel")?;
        let topic = Topic::new(
            line.args[3].decode(),
            line.args[1]
                .decode()
                .parse::<u32>()
                .map_err(|_| "invalid ts")?,
            line.args[2].decode().as_str(),
        )?;
        channel.topic = Some(topic);

        Ok(Outcome::Empty)
    }
}
