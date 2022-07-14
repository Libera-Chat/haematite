use std::collections::{HashMap, HashSet};

use crate::channel::{Channel, Membership};
use crate::network::{Error, Network};
use crate::server::Server;
use crate::user::User;
use crate::util::{NoneOr as _, TrueOr as _};

pub fn add_channel(network: &mut Network, name: Vec<u8>, channel: Channel) -> Result<(), Error> {
    network
        .channels
        .insert(name.clone(), channel)
        .none_or(Error::OverwrittenChannel)?;
    network
        .channel_users
        .insert(name, HashSet::default())
        .none_or(Error::OverwrittenChannel)?;

    Ok(())
}

pub fn del_channel(network: &mut Network, channel: &[u8]) -> Result<(), Error> {
    network
        .channels
        .remove(channel)
        .ok_or(Error::UnknownChannel)?;
    network
        .channel_users
        .remove(channel)
        .ok_or(Error::UnknownChannel)?;

    Ok(())
}

fn chk_channel(network: &mut Network, channel_name: &[u8]) -> Result<(), Error> {
    let channel = network
        .channels
        .get(channel_name)
        .ok_or(Error::UnknownChannel)?;
    let channel_users = network
        .channel_users
        .get(channel_name)
        .ok_or(Error::UnknownChannel)?;

    if channel_users.is_empty() && !channel.modes.contains_key(&'P') {
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
        .user_channels
        .get_mut(&uid)
        .ok_or(Error::UnknownUser)?
        .insert(channel.to_owned(), membership)
        .none_or(Error::OverwrittenChannel)?;
    network
        .channel_users
        .get_mut(channel)
        .ok_or(Error::UnknownChannel)?
        .insert(uid)
        .true_or(Error::OverwrittenUser)?;

    Ok(())
}

pub fn del_user_channel(network: &mut Network, uid: &[u8], channel: &[u8]) -> Result<(), Error> {
    network
        .user_channels
        .get_mut(uid)
        .ok_or(Error::UnknownUser)?
        .remove(channel)
        .ok_or(Error::UnknownChannel)?;
    network
        .channel_users
        .get_mut(channel)
        .ok_or(Error::UnknownChannel)?
        .remove(uid)
        .true_or(Error::UnknownUser)?;

    chk_channel(network, channel)?;

    Ok(())
}

fn add_user_server(network: &mut Network, uid: Vec<u8>, sid: Vec<u8>) -> Result<(), Error> {
    network
        .server_users
        .get_mut(&sid)
        .ok_or(Error::UnknownServer)?
        .insert(uid.clone())
        .true_or(Error::OverwrittenUser)?;
    network
        .user_server
        .insert(uid, sid)
        .none_or(Error::OverwrittenUser)?;

    Ok(())
}

fn del_user_server(network: &mut Network, uid: &[u8]) -> Result<(), Error> {
    let sid = network.user_server.remove(uid).ok_or(Error::UnknownUser)?;
    network
        .server_users
        .get_mut(&sid)
        .ok_or(Error::UnknownServer)?;

    Ok(())
}

pub fn add_user(
    network: &mut Network,
    uid: Vec<u8>,
    sid: Vec<u8>,
    user: User,
) -> Result<(), Error> {
    network
        .users
        .insert(uid.clone(), user)
        .none_or(Error::OverwrittenUser)?;
    network
        .user_channels
        .insert(uid.clone(), HashMap::default())
        .none_or(Error::OverwrittenUser)?;

    add_user_server(network, uid, sid)?;

    Ok(())
}

pub fn del_user(network: &mut Network, uid: &[u8]) -> Result<(), Error> {
    network.users.remove(uid).ok_or(Error::UnknownUser)?;

    let channels = network
        .user_channels
        .remove(uid)
        .ok_or(Error::UnknownUser)?;
    for channel in channels.keys() {
        del_user_channel(network, uid, channel)?;
    }

    del_user_server(network, uid)?;

    Ok(())
}

pub fn add_server(network: &mut Network, sid: Vec<u8>, server: Server) -> Result<(), Error> {
    network
        .servers
        .insert(sid.clone(), server)
        .none_or(Error::OverwrittenServer)?;
    network
        .server_users
        .insert(sid, HashSet::default())
        .none_or(Error::OverwrittenServer)?;

    Ok(())
}

pub fn del_server(network: &mut Network, sid: &[u8]) -> Result<(), Error> {
    network.servers.remove(sid).ok_or(Error::UnknownServer)?;

    let uids = network
        .server_users
        .remove(sid)
        .ok_or(Error::UnknownServer)?;
    for uid in uids {
        del_user(network, &uid)?;
    }

    Ok(())
}
