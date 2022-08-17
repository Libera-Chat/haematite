mod su;

use crate::handler::{Error, Outcome};
use crate::line::Line;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2..)?;

    match line.args[1].as_slice() {
        b"SU" => su::handle(line),
        _ => Ok(Outcome::Unhandled),
    }
}
