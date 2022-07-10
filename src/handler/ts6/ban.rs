use crate::ban::Ban;
use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::oper::Oper;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_ban(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let args: &[Vec<u8>; 8] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing argument")?;

        let btype = args[0][0] as char;
        let mask = match btype {
            'K' => format!("{}@{}", args[1].decode(), args[2].decode()),
            // throw or something instead. only expecting K here
            _ => "asd".to_string(),
        };
        let since = args[3].decode().parse::<u64>().unwrap();
        let duration = args[4].decode().parse::<u64>().unwrap();
        let setter = Oper::from(&args[6].decode());
        let reason = args[7].decode();

        let bans = network.bans.entry(btype).or_insert_with(Default::default);
        let ban = Ban::new(reason, since, duration, setter);

        match duration {
            0 => bans.remove(&mask),
            _ => bans.insert(mask, ban),
        };

        Ok(Outcome::Empty)
    }
}
