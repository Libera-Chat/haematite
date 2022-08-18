use haematite_models::irc::network::Diff as NetDiff;
use haematite_models::irc::user::{Action as UserAction, Diff as UserDiff};

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 2)?;

    let uid = line.args[0].decode();

    let mut deopered = false;
    let mut diff = Vec::new();
    for (mode, remove) in split_chars(&line.args[1].decode()) {
        let action = if remove {
            deopered |= mode == 'o';
            UserAction::Remove
        } else {
            UserAction::Add
        };

        diff.push(NetDiff::InternalUser(
            uid.clone(),
            UserDiff::Mode(mode, action),
        ));
    }

    if deopered {
        // they've lost umode +o, thus are no longer an oper
        diff.push(NetDiff::InternalUser(uid, UserDiff::Oper(None)));
    }

    Ok(Outcome::State(diff))
}
