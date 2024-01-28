use chrono::{Duration as ChronoDuration, NaiveDateTime, Utc};
use haematite_events::EventStore;
use haematite_models::irc::ban::Ban;
use haematite_models::irc::hostmask::{Error as HostmaskError, Hostmask};
use haematite_models::irc::network::Network;
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

pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 8)?;

    if line.args[0][0] != b'K' {
        // we only expect k-lines in BAN
        return Err(Error::InvalidArgument);
    }

    let since = NaiveDateTime::from_timestamp(
        line.args[3]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::InvalidArgument)?,
        0,
    );

    let duration = ChronoDuration::seconds(
        line.args[4]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::InvalidArgument)?,
    );

    let mask = format!("{}@{}", line.args[1].decode(), line.args[2].decode());

    event_store.store(
        "ban.add",
        haematite_models::event::network::AddBan { mask: &mask },
    )?;

    if duration.is_zero() {
        if network.bans.contains_key(&mask) {
            network.bans.remove(&mask);
        }
    } else if since + duration < Utc::now().naive_utc() {
        // expired
    } else {
        let setter =
            parse_oper(line.args[6].decode().as_str()).map_err(|_| Error::InvalidArgument)?;
        let reason = line.args[7].decode();

        let ban = Ban {
            reason,
            since,
            duration: duration.to_std().map_err(|_| Error::InvalidArgument)?,
            setter,
        };

        network.bans.insert(mask, ban);
    };

    Ok(Outcome::Handled)
}
