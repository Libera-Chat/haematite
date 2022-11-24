use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use haematite_models::irc::network::Network;

use super::CAPABS;
use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

pub(super) struct Handler {
    uplink_capabs: Rc<RefCell<HashSet<String>>>,
}

impl Handler {
    pub fn resolver(uplink_capabs: Rc<RefCell<HashSet<String>>>) -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(1, 1, Self { uplink_capabs })
    }
}

impl LineHandler for Handler {
    //CAPAB :BAN CHW CLUSTER EBMASK ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let uplink_capabs: HashSet<String> = line.args[0]
            .split(|c| c == &b' ')
            .map(DecodeHybrid::decode)
            .into_iter()
            .collect();

        let our_capabs = HashSet::from_iter(CAPABS.map(ToString::to_string));

        self.uplink_capabs
            .replace(uplink_capabs.union(&our_capabs).cloned().collect());

        Ok(Outcome::Empty)
    }
}
