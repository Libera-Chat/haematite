mod away;
mod ban;
mod bmask;
mod chghost;
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

use std::time::SystemTime;

use haematite_models::network::Network;
use regex::Regex;

use crate::config::{Config, Error as ConfigError};
use crate::handler::{Error, Handler, Outcome};
use crate::line::Line;

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
            Err(ConfigError::InvalidData("server id".to_string()))
        } else if !regex_name.is_match(&config.server.name) {
            Err(ConfigError::InvalidData("server name".to_string()))
        } else {
            Ok(())
        }
    }

    fn get_burst<'a>(
        &self,
        network: &Network,
        password: &'a str,
    ) -> Result<Vec<String>, &'static str> {
        let now = SystemTime::now();
        let me = &network.servers[&network.me];

        Ok(vec![
            format!("PASS {} TS 6 :{}", password, me.id),
            "CAPAB :BAN CHW CLUSTER ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN".to_string(),
            format!(
                "SERVER {} 1 :{}",
                me.name, me.description
            ),
            format!("SVINFO 6 6 0 {}", now.duration_since(SystemTime::UNIX_EPOCH).map_err(|_e| "GRAN PROBLEMA DE TIEMPO")?.as_secs()),
        ])
    }

    fn handle(&mut self, network: &mut Network, line: &[u8]) -> Result<Outcome, Error> {
        let line = Line::from(line)?;

        match line.command.as_slice() {
            b"AWAY" => Self::handle_away(network, &line),
            b"BAN" => Self::handle_ban(network, &line),
            b"BMASK" => Self::handle_bmask(network, &line),
            b"CHGHOST" => Self::handle_chghost(network, &line),
            b"EUID" => Self::handle_euid(network, &line),
            b"JOIN" => Self::handle_join(network, &line),
            b"KILL" => Self::handle_kill(network, &line),
            b"MODE" => Self::handle_mode(network, &line),
            b"OPER" => Self::handle_oper(network, &line),
            b"PART" => Self::handle_part(network, &line),
            b"PASS" => self.handle_pass(network, &line),
            b"PING" => Self::handle_ping(network, &line),
            b"QUIT" => Self::handle_quit(network, &line),
            b"SERVER" => self.handle_server(network, &line),
            b"SID" => Self::handle_sid(network, &line),
            b"SJOIN" => Self::handle_sjoin(network, &line),
            b"SQUIT" => Self::handle_squit(network, &line),
            b"TB" => Self::handle_tb(network, &line),
            b"TMODE" => Self::handle_tmode(network, &line),
            b"TOPIC" => Self::handle_topic(network, &line),
            _ => Ok(Outcome::Unhandled),
        }
    }
}