use regex::Regex;

use crate::ban::Ban;
use crate::handler::{Error, Outcome};
use crate::hostmask::{Error as HostmaskError, Hostmask};
use crate::line::Line;
use crate::network::Network;
use crate::oper::Oper;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

fn parse_oper(mut oper: &str) -> Result<Oper, HostmaskError> {
    //TODO: precompile this
    let oper_regex = Regex::new(r"^([^{]+)\{(\S+)\}$").unwrap();

    let hostmask = match oper_regex.captures(oper) {
        Some(hmatch) => {
            let hostmask = hmatch.get(1).unwrap().as_str();
            oper = hmatch.get(2).unwrap().as_str();
            Some(Hostmask::try_from(hostmask)?)
        }
        None => None,
    };

    Ok(Oper {
        name: oper.to_string(),
        hostmask,
    })
}

impl TS6Handler {
    pub fn handle_ban(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() != 8 {
            return Err(Error::ExpectedArguments(8));
        }

        let btype = line.args[0][0] as char;
        let mask = match btype {
            'K' => format!("{}@{}", line.args[1].decode(), line.args[2].decode()),
            // throw or something instead. only expecting K here
            _ => "asd".to_string(),
        };
        let since = line.args[3].decode().parse::<u64>().unwrap();
        let duration = line.args[4].decode().parse::<u64>().unwrap();
        let setter = parse_oper(line.args[6].decode().as_str()).map_err(|_| Error::BadArgument)?;
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
