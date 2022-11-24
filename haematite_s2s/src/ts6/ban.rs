use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use haematite_models::irc::ban::Ban;
use haematite_models::irc::hostmask::{Error as HostmaskError, Hostmask};
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::oper::Oper;
use regex::Regex;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

enum OperError {
    BadFormat,
    BadHostmask,
}

impl From<HostmaskError> for OperError {
    fn from(_error: HostmaskError) -> Self {
        Self::BadHostmask
    }
}

fn parse_oper(oper: &str) -> Result<Oper, OperError> {
    //TODO: precompile this
    let oper_regex = Regex::new(r"^([^{]+)\{(\S+)\}$").unwrap();

    let hmatch = oper_regex.captures(oper).ok_or(OperError::BadFormat)?;
    let hostmask = Hostmask::try_from(hmatch.get(1).unwrap().as_str())?;
    let name = hmatch.get(2).unwrap().as_str().to_string();

    Ok(Oper { name, hostmask })
}

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(8, 8, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        if line.args[0][0] != b'K' {
            // we only expect k-lines in BAN
            return Err(Error::UnexpectedValue);
        }

        let since = line.args[3]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::InvalidNumber)?;

        let duration = line.args[4]
            .decode()
            .parse::<u32>()
            .map_err(|_| Error::InvalidNumber)?;

        let mask = format!("{}@{}", line.args[1].decode(), line.args[2].decode());

        let action = if duration == 0 {
            NetAction::Remove
        } else if (Utc::now().timestamp() - since) > duration.into() {
            // expired
            return Ok(Outcome::Empty);
        } else {
            let setter =
                parse_oper(line.args[6].decode().as_str()).map_err(|_| Error::InvalidFormat)?;
            let reason = line.args[7].decode();

            let ban = Ban {
                reason,
                since: NaiveDateTime::from_timestamp(since, 0),
                duration: Duration::from_secs(duration.into()),
                setter,
            };

            NetAction::Add(ban)
        };

        Ok(Outcome::State(vec![NetDiff::Ban(mask, action)]))
    }
}
