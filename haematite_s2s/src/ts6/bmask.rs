use haematite_models::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_bmask(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Line::assert_arg_count(line, 4)?;

        let channel = network.get_channel_mut(&line.args[1])?;
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
