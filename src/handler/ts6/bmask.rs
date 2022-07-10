use crate::handler::Outcome;
use crate::line::Line;
use crate::network::Network;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_bmask(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let args: &[Vec<u8>; 4] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing arugment")?;

        let channel = network.get_channel_mut(&args[1].decode());
        let mode = args[2][0] as char;
        let masks_new = args[3].split(|c| c == &b' ');

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
