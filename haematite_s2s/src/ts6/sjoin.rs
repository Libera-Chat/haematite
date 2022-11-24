use std::collections::{HashMap, HashSet};

use haematite_models::irc::channel::Channel;
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff, Network};
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use super::util::mode::to_changes;
use crate::handler::{ArgumentCountResolver, Error, LineHandler, LineHandlerResolver, Outcome};
use crate::line::Line;
use crate::util::mode::{split_chars, ArgType};
use crate::util::DecodeHybrid;

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_handler(4, 4, Self {})
    }
}

impl LineHandler for Handler {
    //:00A SJOIN 1658071435 #services +nst :@00AAAAAAB
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error> {
        let channel_name = line.args[1].decode();
        let uids = line.args[line.args.len() - 1]
            .split(|c| c == &b' ')
            // we may get an empty last param
            .filter(|a| !a.is_empty())
            .collect::<Vec<&[u8]>>();

        let mut diff = Vec::new();

        let mut new_channel = Channel::new(HashSet::from_iter(vec!['I', 'b', 'e', 'q']));

        if let Some(channel) = network.channels.get(&channel_name) {
            for nick in channel.users.keys() {
                new_channel.users.insert(nick.clone(), Membership::new());
            }
        }

        let modes = to_changes(split_chars(&line.args[2].decode()));
        let mut args = line.args[3..line.args.len() - 1].iter();
        let args_total = args.len();

        for (i, change) in modes.iter().enumerate() {
            let arg = match change.arg_type {
                ArgType::None => None,
                _ => Some(args.next().map(DecodeHybrid::decode).ok_or_else(|| {
                    Error::InsufficientArguments {
                        expected: i,
                        actual: args_total,
                    }
                })?),
            };
            new_channel.modes.insert(change.mode, arg);
        }

        for uid in uids {
            //TODO: precompile
            let statuses = HashMap::from([('+', 'v'), ('@', 'o')]);

            let mut uid = uid.decode();

            let mut membership = Membership::new();

            while let Some(char) = uid.chars().next() {
                if let Some(mode) = statuses.get(&char) {
                    membership.status.push(*mode);
                    uid.remove(0);
                } else {
                    break;
                }
            }

            if uid.is_empty() {
                return Err(Error::EmptyArgument);
            }

            diff.push(NetDiff::InternalUser(
                uid.clone(),
                UserDiff::Channel(channel_name.clone(), UserAction::Add),
            ));
            new_channel.users.insert(uid, membership);
        }

        diff.insert(
            0,
            NetDiff::ExternalChannel(channel_name, NetAction::Add(new_channel)),
        );

        Ok(Outcome::State(diff))
    }
}
