mod away;
mod ban;
mod bmask;
mod capab;
mod chghost;
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

use std::collections::HashSet;
use std::time::SystemTime;

use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::irc::network::Network;
use regex::Regex;

use crate::handler::{Error, Handler, Outcome};
use crate::line::Line;

const CAPABS: [&str; 19] = [
    "BAN", "CHW", "CLUSTER", "EBMASK", "ECHO", "ENCAP", "EOPMOD", "EUID", "EX", "IE", "KLN",
    "KNOCK", "MLOCK", "QS", "RSFNC", "SAVE", "SERVICES", "TB", "UNKLN",
];

#[derive(Default)]
pub struct TS6Handler {
    uplink: Option<Vec<u8>>,
    uplink_capabs: HashSet<String>,
}

impl TS6Handler {
    pub fn new() -> Self {
        TS6Handler::default()
    }
}

impl Handler for TS6Handler {
    fn validate_config(&self, config: &Config) -> Result<(), ConfigError> {
        //TODO: precompile
        let regex_sid = Regex::new(r"^[0-9][0-9A-Z]{2}$").unwrap();
        let regex_name = Regex::new(r"^[0-9a-zA-Z]+\.[0-9a-zA-Z\.]*$").unwrap();

        if !regex_sid.is_match(&config.server.id) {
            Err(ConfigError::InvalidId)
        } else if !regex_name.is_match(&config.server.name) {
            Err(ConfigError::InvalidName)
        } else {
            Ok(())
        }
    }

    fn get_burst<'a>(&self, network: &Network, password: &'a str) -> Result<Vec<String>, String> {
        let now = SystemTime::now();
        let me = &network.servers[&network.me];

        Ok(vec![
            format!("PASS {} TS 6 :{}", password, me.id),
            format!("CAPAB :{}", CAPABS.join(" ")),
            format!("SERVER {} 1 :{}", me.name, me.description),
            format!(
                "SVINFO 6 6 0 {}",
                now.duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|_e| "GRAN PROBLEMA DE TIEMPO".to_string())?
                    .as_secs()
            ),
        ])
    }

    fn handle(&mut self, network: &Network, line: &[u8]) -> Result<Outcome, Error> {
        let line = Line::try_from_rfc1459(line)?;

        match line.command.as_slice() {
            b"AWAY" => away::handle(&line),
            b"BAN" => ban::handle(&line),
            b"BMASK" => bmask::handle(&line),
            b"CAPAB" => capab::handle(self, &line),
            b"CHGHOST" => chghost::handle(&line),
            b"ENCAP" => encap::handle(&line),
            b"EUID" => euid::handle(&line),
            b"JOIN" => join::handle(&line),
            b"KILL" => kill::handle(&network, &line),
            b"MODE" => mode::handle(&line),
            b"NICK" => nick::handle(&line),
            b"OPER" => oper::handle(&line),
            b"PART" => part::handle(&network, &line),
            b"PASS" => pass::handle(self, &line),
            b"PING" => ping::handle(&network, &line),
            b"QUIT" => quit::handle(&network, &line),
            b"SERVER" => server::handle(self, &line),
            b"SID" => sid::handle(&line),
            b"SJOIN" => sjoin::handle(&network, &line),
            b"SQUIT" => squit::handle(&network, &line),
            b"TB" => tb::handle(&line),
            b"TMODE" => tmode::handle(&line),
            b"TOPIC" => topic::handle(&network, &line),
            _ => Ok(Outcome::Unhandled),
        }
    }
}
