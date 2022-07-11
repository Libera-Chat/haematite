pub mod ts6;

use crate::line::Line;
use crate::network::Network;

pub enum Outcome {
    Unhandled,
    Empty,
    Response(Vec<String>),
}

pub trait Handler {
    fn get_burst<'a>(
        &self,
        network: &Network,
        password: &'a str,
    ) -> Result<Vec<String>, &'static str>;
    fn handle(&mut self, network: &mut Network, line: Line) -> Result<Outcome, &'static str>;
}
