use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_bmask(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        if line.args.len() != 4 {
            return Err(Error::ExpectedArguments(4));
        }

        let channel = network
            .channels
            .get_mut(&line.args[1])
            .ok_or(Error::UnknownChannel)?;
        let mode = line.args[2][0] as char;
        let masks_new = line.args[3].split(|c| c == &b' ');

        let masks = channel
            .mode_lists
            .entry(mode)
            .or_insert_with(Default::default);

        for mask in masks_new {
            masks.insert(mask.decode());
        }

        Ok(Outcome::Empty)
    }
}
