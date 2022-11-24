use std::cell::RefCell;
use std::rc::Rc;

use haematite_models::irc::network::Network;

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;

pub(super) struct Handler {
    uplink: Rc<RefCell<Option<Vec<u8>>>>,
}

impl Handler {
    pub fn resolver(uplink: Rc<RefCell<Option<Vec<u8>>>>) -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(4, 4, Self { uplink })
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        self.uplink.replace(Some(line.args[3].clone()));
        Ok(Outcome::Empty)
    }
}
