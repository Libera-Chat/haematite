use crate::channel_user::ChannelUser;
use std::collections::HashMap;

#[derive(Default)]
pub struct Channel {
    users: HashMap<String, ChannelUser>,
}

impl Channel {
    pub fn new(users: impl Iterator<Item = String>) -> Self {
        //TODO: turn `users` in to HashMap
        Channel {
            users: users.map(|u| (u, ChannelUser::new())).into_iter().collect(),
        }
    }

    pub fn add_user(mut self, uid: String, channel_user: ChannelUser) -> bool {
        self.users.insert(uid, channel_user).is_none()
    }
}
