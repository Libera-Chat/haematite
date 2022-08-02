use haematite_models::irc::channel::Channel;
use haematite_models::irc::membership::Membership;
use haematite_models::irc::network::{Error, Network};
use haematite_models::irc::server::Server;
use haematite_models::irc::user::User;

use crate::util::{NoneOr as _, TrueOr as _};

pub fn add_channel(network: &mut Network, name: String, channel: Channel) -> Result<(), Error> {
    network
        .channels
        .insert(name, channel)
        .none_or(Error::OverwrittenChannel)?;

    Ok(())
}

pub fn del_channel(network: &mut Network, channel: &str) -> Result<(), Error> {
    network
        .channels
        .remove(channel)
        .ok_or(Error::UnknownChannel)?;

    Ok(())
}

fn chk_channel(network: &mut Network, channel_name: &str) -> Result<(), Error> {
    let channel = network
        .channels
        .get(channel_name)
        .ok_or(Error::UnknownChannel)?;

    if channel.users.is_empty() && !channel.modes.contains_key(&'P') {
        del_channel(network, channel_name)?;
    }

    Ok(())
}

pub fn add_user_channel(
    network: &mut Network,
    uid: String,
    // lint complains that `channel` isn't owned after the function,
    // so it's a &str, not a String
    channel: &str,
    membership: Membership,
) -> Result<(), Error> {
    network
        .users
        .get_mut(&uid)
        .ok_or(Error::UnknownUser)?
        .channels
        .insert(channel.to_owned())
        .true_or(Error::OverwrittenChannel)?;
    network
        .channels
        .get_mut(channel)
        .ok_or(Error::UnknownChannel)?
        .users
        .insert(uid, membership)
        .none_or(Error::OverwrittenUser)?;

    Ok(())
}

pub fn del_user_channel(network: &mut Network, uid: &str, channel: &str) -> Result<(), Error> {
    network
        .users
        .get_mut(uid)
        .ok_or(Error::UnknownUser)?
        .channels
        .remove(channel)
        .true_or(Error::UnknownChannel)?;
    network
        .channels
        .get_mut(channel)
        .ok_or(Error::UnknownChannel)?
        .users
        .remove(uid)
        .ok_or(Error::UnknownUser)?;

    chk_channel(network, channel)?;

    Ok(())
}

pub fn add_user(network: &mut Network, uid: String, user: User) -> Result<(), Error> {
    let sid = user.server.clone();

    network
        .users
        .insert(uid.clone(), user)
        .none_or(Error::OverwrittenUser)?;

    network
        .servers
        .get_mut(&sid)
        .ok_or(Error::UnknownServer)?
        .users
        .insert(uid)
        .true_or(Error::OverwrittenUser)?;

    Ok(())
}

pub fn del_user(network: &mut Network, uid: &str) -> Result<(), Error> {
    let user_im = network.get_user(uid)?;

    for channel in user_im.channels.clone() {
        del_user_channel(network, uid, &channel)?;
    }

    let user = network.users.remove(uid).unwrap();

    network
        .servers
        .get_mut(&user.server)
        .ok_or(Error::UnknownServer)?
        .users
        .remove(uid)
        .true_or(Error::UnknownUser)?;

    Ok(())
}

pub fn add_server(network: &mut Network, sid: String, server: Server) -> Result<(), Error> {
    network
        .servers
        .insert(sid, server)
        .none_or(Error::OverwrittenServer)?;

    Ok(())
}

pub fn del_server(network: &mut Network, sid: &str) -> Result<(), Error> {
    let server = network.get_server(sid)?;

    for uid in server.users.clone() {
        del_user(network, &uid)?;
    }

    network.servers.remove(sid).unwrap();

    Ok(())
}
