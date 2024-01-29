use chrono::{Duration as ChronoDuration, NaiveDateTime, Utc};
use haematite_events::EventStore;
use haematite_models::irc::ban::Ban;
use haematite_models::irc::hostmask::{Error as HostmaskError, Hostmask};
use haematite_models::irc::network::Network;
use haematite_models::irc::oper::Oper;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

enum OperStage {
    Oper,
    Hostmask,
}

enum ParseOperError {
    Hostmask(HostmaskError),
    Format,
}

impl From<HostmaskError> for ParseOperError {
    fn from(value: HostmaskError) -> Self {
        Self::Hostmask(value)
    }
}

fn parse_oper(value: &str) -> Result<Oper, ParseOperError> {
    let mut hostmask = String::new();
    let mut opername = String::new();
    let mut stage = OperStage::Hostmask;

    for char in value.chars() {
        match stage {
            OperStage::Hostmask => {
                if char == '{' {
                    stage = OperStage::Oper;
                } else {
                    hostmask.push(char);
                }
            }
            OperStage::Oper => {
                if char == '}' {
                    break;
                }
                opername.push(char);
            }
        };
    }

    if opername.is_empty() {
        Err(ParseOperError::Format)
    } else {
        Ok(Oper {
            name: opername.to_string(),
            hostmask: Some(Hostmask::try_from(hostmask.as_str())?),
        })
    }
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
    let expires = since + duration;
    if duration.is_zero() {
        if network.bans.contains_key(&mask) {
            network.bans.remove(&mask);
        }
    } else if expires < Utc::now().naive_utc() {
        // expired
    } else {
        let setter =
            parse_oper(line.args[6].decode().as_str()).map_err(|_| Error::InvalidArgument)?;
        let reason = line.args[7].decode();

        let ban = Ban {
            expires,
            reason,
            setter,
            since,
        };

        event_store.store(
            "ban.add",
            haematite_models::event::network::AddBan {
                expires: &ban.expires,
                mask: &mask,
                reason: &ban.reason,
                setter: &ban.setter,
                since: &ban.since,
            },
        )?;

        network.bans.insert(mask, ban);
    };

    Ok(Outcome::Handled)
}
