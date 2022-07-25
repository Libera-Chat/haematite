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
use haematite_models::network::Network;
use regex::Regex;

use crate::handler::{Error, Handler, Outcome};
use crate::line::Line;

const CAPABS: [&str; 19] = [
    "BAN", "CHW", "CLUSTER", "EBMASK", "ECHO", "ENCAP", "EOPMOD", "EUID", "EX", "IE", "KLN",
    "KNOCK", "MLOCK", "QS", "RSFNC", "SAVE", "SERVICES", "TB", "UNKLN",
];

fn parse_mode_args<'a>(
    modes: impl Iterator<Item = (char, bool)>,
    mut args: impl Iterator<Item = &'a Vec<u8>>,
) -> impl Iterator<Item = (char, bool, Option<&'a Vec<u8>>)> {
    let mut out = Vec::new();

    for (mode, remove) in modes {
        let arg = match mode {
            'k' => true,
            'f' | 'j' | 'l' if !remove => true,
            _ => false,
        }
        //TODO: Resultify this unwrap
        .then(|| args.next().unwrap());

        out.push((mode, remove, arg));
    }

    out.into_iter()
}

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

    fn handle(&mut self, network: &mut Network, line: &[u8]) -> Result<Outcome, Error> {
        let line = Line::try_from_rfc1459(line)?;

        match line.command.as_slice() {
            b"AWAY" => away::handle(network, &line),
            b"BAN" => ban::handle(network, &line),
            b"BMASK" => bmask::handle(network, &line),
            b"CAPAB" => capab::handle(self, network, &line),
            b"CHGHOST" => chghost::handle(network, &line),
            b"ENCAP" => encap::handle(network, &line),
            b"EUID" => euid::handle(network, &line),
            b"JOIN" => join::handle(network, &line),
            b"KILL" => kill::handle(network, &line),
            b"MODE" => mode::handle(network, &line),
            b"OPER" => oper::handle(network, &line),
            b"PART" => part::handle(network, &line),
            b"PASS" => pass::handle(self, network, &line),
            b"PING" => ping::handle(network, &line),
            b"QUIT" => quit::handle(network, &line),
            b"SERVER" => server::handle(self, network, &line),
            b"SID" => sid::handle(network, &line),
            b"SJOIN" => sjoin::handle(network, &line),
            b"SQUIT" => squit::handle(network, &line),
            b"TB" => tb::handle(network, &line),
            b"TMODE" => tmode::handle(network, &line),
            b"TOPIC" => topic::handle(network, &line),
            _ => Ok(Outcome::Unhandled),
        }
    }
}
