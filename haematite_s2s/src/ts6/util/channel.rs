use haematite_models::irc::channel::Channel;

pub enum ForgetContext {
    Leave(u8),
}

pub trait Forgettable {
    fn is_forgettable(&self, context: ForgetContext) -> bool;
}

impl Forgettable for Channel {
    fn is_forgettable(&self, context: ForgetContext) -> bool {
        if self.modes.contains_key(&'P') {
            return false;
        }

        match context {
            // this could have been `<=` instead of `=`, but if we're trying
            // to remove more users than we have, there's a bigger problem
            ForgetContext::Leave(count) => self.users.len() == count.into(),
        }
    }
}
