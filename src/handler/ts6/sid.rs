use std::collections::HashSet;

use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::{DecodeHybrid as _, NoneOr as _};

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_sid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        if line.args.len() != 4 {
            return Err("unexpected argument count");
        }

        let sid = &line.args[2];
        network
            .servers
            .insert(
                sid.clone(),
                Server::new(sid.decode(), line.args[0].decode(), line.args[3].decode()),
            )
            .none_or("overwriten sid")?;
        network
            .server_users
            .insert(sid.clone(), HashSet::default())
            .none_or("overwriten sid")?;

        Ok(Outcome::Empty)
    }
}
