pub mod ts6;

use std::net::TcpStream;

use crate::line::Line;
use crate::network::Network;

pub trait Handler {
    fn get_burst<'a>(&self, network: &Network, password: &'a str) -> Vec<String>;
    fn handle(&mut self, network: &mut Network, socket: &TcpStream, line: &Line) -> bool;
}
