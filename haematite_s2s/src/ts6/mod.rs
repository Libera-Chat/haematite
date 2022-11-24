mod away;
mod ban;
mod bmask;
mod capab;
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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::SystemTime;

use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::irc::network::Network;
use regex::Regex;

use crate::handler::{Error, Handler, LineHandlerResolution, LineHandlerResolver, Outcome};
use crate::line::Line;

const CAPABS: [&str; 18] = [
    "BAN", "CHW", "CLUSTER", "EBMASK", "ECHO", "ENCAP", "EOPMOD", "EUID", "EX", "IE", "KLN",
    "KNOCK", "MLOCK", "QS", "RSFNC", "SERVICES", "TB", "UNKLN",
];

#[derive(Default)]
pub struct TS6Handler {
    handler_resolvers: HashMap<&'static [u8], Box<dyn LineHandlerResolver>>,
    uplink: Rc<RefCell<Option<Vec<u8>>>>,
    uplink_capabs: Rc<RefCell<HashSet<String>>>,
}

impl TS6Handler {
    pub fn new() -> Self {
        let mut handler = Self::default();
        handler.add_line_handler_resolver(b"AWAY", away::Handler::resolver());
        handler.add_line_handler_resolver(b"BAN", ban::Handler::resolver());
        handler.add_line_handler_resolver(b"BMASK", bmask::Handler::resolver());
        handler.add_line_handler_resolver(
            b"CAPAB",
            capab::Handler::resolver(Rc::clone(&handler.uplink_capabs)),
        );
        handler.add_line_handler_resolver(b"CHGHOST", chghost::Handler::resolver());
        handler.add_line_handler_resolver(b"EBMASK", ebmask::Handler::resolver());
        handler.add_line_handler_resolver(b"ENCAP", encap::Handler::resolver());
        handler.add_line_handler_resolver(b"EUID", euid::Handler::resolver());
        handler.add_line_handler_resolver(b"JOIN", join::Handler::resolver());
        handler.add_line_handler_resolver(b"KILL", kill::Handler::resolver());
        handler.add_line_handler_resolver(b"MODE", mode::Handler::resolver());
        handler.add_line_handler_resolver(b"NICK", nick::Handler::resolver());
        handler.add_line_handler_resolver(b"OPER", oper::Handler::resolver());
        handler.add_line_handler_resolver(b"PART", part::Handler::resolver());
        handler.add_line_handler_resolver(
            b"PASS",
            pass::Handler::resolver(Rc::clone(&handler.uplink)),
        );
        handler.add_line_handler_resolver(b"PING", ping::Handler::resolver());
        handler.add_line_handler_resolver(b"QUIT", quit::Handler::resolver());
        handler.add_line_handler_resolver(
            b"SERVER",
            server::Handler::resolver(Rc::clone(&handler.uplink)),
        );
        handler.add_line_handler_resolver(b"SID", sid::Handler::resolver());
        handler.add_line_handler_resolver(b"SJOIN", sjoin::Handler::resolver());
        handler.add_line_handler_resolver(b"SQUIT", squit::Handler::resolver());
        handler.add_line_handler_resolver(b"TB", tb::Handler::resolver());
        handler.add_line_handler_resolver(b"TMODE", tmode::Handler::resolver());
        handler.add_line_handler_resolver(b"TOPIC", topic::Handler::resolver());
        handler
    }
}

impl TS6Handler {
    fn add_line_handler_resolver<T: LineHandlerResolver + 'static>(
        &mut self,
        command: &'static [u8],
        handler: T,
    ) {
        self.handler_resolvers.insert(command, Box::new(handler));
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

        if let Some(handler_resolver) = self.handler_resolvers.get_mut(line.command.as_slice()) {
            let mut handler_resolver = handler_resolver;
            loop {
                match handler_resolver.resolve(network, &line)? {
                    Some(LineHandlerResolution::SeeOther(resolver_inner)) => {
                        handler_resolver = resolver_inner;
                    }
                    Some(LineHandlerResolution::Handler(handler)) => {
                        break handler.handle(network, &line);
                    }
                    None => break Ok(Outcome::Unhandled),
                };
            }
        } else {
            Ok(Outcome::Unhandled)
        }
    }
}
