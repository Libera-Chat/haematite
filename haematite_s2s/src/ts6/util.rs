use haematite_models::channel::{Channel, Membership};
use haematite_models::network::{Error, Network};
use haematite_models::server::Server;
use haematite_models::user::User;

use crate::util::{NoneOr as _, TrueOr as _};

pub fn add_channel(network: &mut Network, name: Vec<u8>, channel: Channel) -> Result<(), Error> {
    network
        .channels
        .insert(name, channel)
        .none_or(Error::OverwrittenChannel)?;

    Ok(())
}

pub fn del_channel(network: &mut Network, channel: &[u8]) -> Result<(), Error> {
    network
        .channels
        .remove(channel)
        .ok_or(Error::UnknownChannel)?;

    Ok(())
}

fn chk_channel(network: &mut Network, channel_name: &[u8]) -> Result<(), Error> {
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
    uid: Vec<u8>,
    // lint complains that `channel` isn't owned after the function,
    // so it's a ref, not a vec
    channel: &[u8],
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

pub fn del_user_channel(network: &mut Network, uid: &[u8], channel: &[u8]) -> Result<(), Error> {
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

pub fn add_user(network: &mut Network, uid: Vec<u8>, user: User) -> Result<(), Error> {
    let sid = user.server.value.clone();

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

pub fn del_user(network: &mut Network, uid: &[u8]) -> Result<(), Error> {
    let user = network.users.remove(uid).ok_or(Error::UnknownUser)?;

    for channel in user.channels {
        del_user_channel(network, uid, &channel)?;
    }

    network
        .servers
        .get_mut(&user.server.value)
        .ok_or(Error::UnknownServer)?
        .users
        .remove(uid)
        .true_or(Error::UnknownUser)?;

    Ok(())
}

pub fn add_server(network: &mut Network, sid: Vec<u8>, server: Server) -> Result<(), Error> {
    network
        .servers
        .insert(sid, server)
        .none_or(Error::OverwrittenServer)?;

    Ok(())
}

pub fn del_server(network: &mut Network, sid: &[u8]) -> Result<(), Error> {
    let server = network.servers.remove(sid).ok_or(Error::UnknownServer)?;

    for uid in server.users {
        del_user(network, &uid)?;
    }

    Ok(())
}
