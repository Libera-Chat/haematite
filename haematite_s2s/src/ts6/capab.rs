use std::collections::HashSet;

use super::{TS6Handler, CAPABS};
use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::DecodeHybrid;

//CAPAB :BAN CHW CLUSTER EBMASK ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN
pub fn handle(ts6: &mut TS6Handler, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 1)?;

    let uplink_capabs: HashSet<String> = line.args[0]
        .split(|c| c == &b' ')
        .map(DecodeHybrid::decode)
        .into_iter()
        .collect();

    let our_capabs = HashSet::from_iter(CAPABS.map(ToString::to_string));

    ts6.uplink_capabs = uplink_capabs.union(&our_capabs).cloned().collect();

    Ok(Outcome::Empty)
}
