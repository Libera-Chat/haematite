use chrono::Utc;
use haematite_models::irc::channel::Diff as ChanDiff;
use haematite_models::irc::error::Error as StateError;
use haematite_models::irc::hostmask::Hostmask;
use haematite_models::irc::network::{Diff as NetDiff, Network};
use haematite_models::irc::topic::{Setter, Topic};

use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(2, 2, Self {})
    }
}

impl LineHandler for Handler {
    //:420AAAABG TOPIC #test :hi
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let channel_name = line.args[0].decode();
        let text = line.args[1].decode();

        let topic = if text.is_empty() {
            None
        } else {
            let uid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();

            let user = network.users.get(&uid).ok_or(StateError::UnknownUser)?;
            let hostmask = Hostmask {
                nick: user.nick.clone(),
                user: user.user.clone(),
                host: user.host.clone(),
            };

            Some(Topic {
                text,
                since: Utc::now().naive_utc(),
                setter: Setter::Hostmask(hostmask),
            })
        };

        Ok(Outcome::State(vec![NetDiff::InternalChannel(
            channel_name,
            ChanDiff::Topic(topic),
        )]))
    }
}
