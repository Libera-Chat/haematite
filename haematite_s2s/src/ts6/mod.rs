mod away;
mod ban;
mod bmask;
mod chghost;
mod ebmask;
mod encap;
mod euid;
mod join;
mod kill;
mod mode;
mod nick;
mod oper;
mod part;
mod pass;
mod ping;
mod quit;
mod server;
mod sid;
mod sjoin;
mod squit;
mod tb;
mod tmode;
mod topic;
mod util;

use haematite_events::EventStore;
use haematite_models::config::Error as ConfigError;
use haematite_models::irc::network::Network;
use haematite_models::irc::server::Server;
use regex::Regex;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::DecodeHybrid as _;

const CAPABS: [&str; 18] = [
    "BAN", "CHW", "CLUSTER", "EBMASK", "ECHO", "ENCAP", "EOPMOD", "EUID", "EX", "IE", "KLN",
    "KNOCK", "MLOCK", "QS", "RSFNC", "SERVICES", "TB", "UNKLN",
];

#[derive(Default)]
#[must_use]
pub struct Handler {
    me: Server,
    pub times: HashMap<String, Vec<u128>>,
}

impl TryFrom<Server> for Handler {
    type Error = ConfigError;

    fn try_from(server: Server) -> Result<Self, Self::Error> {
        //TODO: precompile
        let regex_sid = Regex::new(r"^[0-9][0-9A-Z]{2}$").unwrap();
        let regex_name = Regex::new(r"^[0-9a-zA-Z]+\.[0-9a-zA-Z\.]*$").unwrap();

        if !regex_sid.is_match(&server.id) {
            Err(ConfigError::InvalidId)
        } else if !regex_name.is_match(&server.name) {
            Err(ConfigError::InvalidName)
        } else {
            Ok(Self {
                me: server,
                times: HashMap::default(),
            })
        }
    }
}

impl crate::handler::Handler for Handler {
    fn get_burst(&self, password: &str) -> Result<Vec<String>, String> {
        let now = SystemTime::now();

        Ok(vec![
            format!("PASS {password} TS 6 :{}", self.me.id),
            format!("CAPAB :{}", CAPABS.join(" ")),
            format!("SERVER {} 1 :{}", self.me.name, self.me.description),
            format!(
                "SVINFO 6 6 0 {}",
                now.duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|e| format!("GRAN PROBLEMA DE TIEMPO: {e}"))?
                    .as_secs()
            ),
        ])
    }

    fn handle<E: EventStore>(
        &mut self,
        event_store: &mut E,
        network: &mut Network,
        line: &[u8],
    ) -> Result<Outcome, Error> {
        let now = std::time::Instant::now();
        let line = Line::try_from_rfc1459(line)?;

        let result = match line.command.as_slice() {
            b"AWAY" => away::handle(event_store, network, &line),
            b"BAN" => ban::handle(event_store, network, &line),
            b"BMASK" => bmask::handle(event_store, network, &line),
            b"CHGHOST" => chghost::handle(event_store, network, &line),
            b"EBMASK" => ebmask::handle(event_store, network, &line),
            b"ENCAP" => encap::handle(event_store, network, &line),
            b"EUID" => euid::handle(event_store, network, &line),
            b"JOIN" => join::handle(event_store, network, &line),
            b"KILL" => kill::handle(event_store, network, &line),
            b"MODE" => mode::handle(event_store, network, &line),
            b"NICK" => nick::handle(event_store, network, &line),
            b"OPER" => oper::handle(event_store, network, &line),
            b"PART" => part::handle(event_store, network, &line),
            b"PASS" => pass::handle(event_store, network, &line),
            b"PING" => ping::handle(event_store, network, &line),
            b"QUIT" => quit::handle(event_store, network, &line),
            b"SERVER" => server::handle(event_store, network, &line),
            b"SID" => sid::handle(event_store, network, &line),
            b"SJOIN" => sjoin::handle(event_store, network, &line),
            b"SQUIT" => squit::handle(event_store, network, &line),
            b"TB" => tb::handle(event_store, network, &line),
            b"TMODE" => tmode::handle(event_store, network, &line),
            b"TOPIC" => topic::handle(event_store, network, &line),
            _ => Ok(Outcome::Unhandled),
        };

        let elapsed = now.elapsed().as_nanos();
        let command = line.command.decode();
        if let Some(times) = self.times.get_mut(&command) {
            times.push(elapsed);
        } else {
            self.times.insert(command, Vec::from([elapsed]));
        }

        result
    }
}
