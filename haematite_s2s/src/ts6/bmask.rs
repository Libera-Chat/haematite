use haematite_models::irc::channel::{Action as ChanAction, Diff as ChanDiff};
use haematite_models::irc::network::{Diff as NetDiff, Network};

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(4, 4, Self {})
    }
}

impl LineHandler for Handler {
    fn handle(&mut self, _network: &Network, line: &Line) -> Result<Outcome, Error> {
        let channel_name = line.args[1].decode();
        let mode = line.args[2][0] as char;
        let masks_new = line.args[3].split(|c| c == &b' ');

        let mut diff = Vec::new();
        for mask in masks_new {
            diff.push(NetDiff::InternalChannel(
                channel_name.clone(),
                ChanDiff::ModeList(mode, mask.decode(), ChanAction::Add(None)),
            ));
        }

        Ok(Outcome::State(diff))
    }
}
