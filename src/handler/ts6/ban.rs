use crate::ban::Ban;
use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::oper::Oper;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_ban(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() != 8 {
            return Err("unexpected argument count");
        }

        let btype = line.args[0][0] as char;
        let mask = match btype {
            'K' => format!("{}@{}", line.args[1].decode(), line.args[2].decode()),
            // throw or something instead. only expecting K here
            _ => "asd".to_string(),
        };
        let since = line.args[3].decode().parse::<u64>().unwrap();
        let duration = line.args[4].decode().parse::<u64>().unwrap();
        let setter = Oper::from(&line.args[6].decode());
        let reason = line.args[7].decode();

        let bans = network.bans.entry(btype).or_insert_with(Default::default);
        let ban = Ban::new(reason, since, duration, setter);

        match duration {
            0 => bans.remove(&mask),
            _ => bans.insert(mask, ban),
        };

        Ok(Outcome::Empty)
    }
}
