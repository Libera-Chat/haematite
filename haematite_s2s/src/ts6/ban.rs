use std::time::Duration;

use chrono::NaiveDateTime;
use haematite_models::irc::ban::Ban;
use haematite_models::irc::hostmask::{Error as HostmaskError, Hostmask};
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff};
use haematite_models::irc::oper::Oper;
use regex::Regex;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

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

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 8)?;

    if line.args[0][0] != b'K' {
        // we only expect k-lines in BAN
        return Err(Error::InvalidArgument);
    }

    let mask = format!("{}@{}", line.args[1].decode(), line.args[2].decode());
    let since = line.args[3].decode();
    let duration = line.args[4].decode();
    let setter = parse_oper(line.args[6].decode().as_str()).map_err(|_| Error::InvalidArgument)?;
    let reason = line.args[7].decode();

    let ban = Ban {
        reason,
        since: NaiveDateTime::from_timestamp(
            since.parse::<i64>().map_err(|_| Error::InvalidArgument)?,
            0,
        ),
        duration: Duration::from_secs(
            duration
                .parse::<u64>()
                .map_err(|_| Error::InvalidArgument)?,
        ),
        setter,
    };

    let action = if duration == *"0" {
        NetAction::Remove
    } else {
        NetAction::Add(ban)
    };

    Ok(Outcome::State(vec![NetDiff::Ban(mask, action)]))
}
