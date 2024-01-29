#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]

mod events;
mod s2s;
pub(crate) mod util;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::{ArgAction, Parser};
use futures::future::TryFutureExt as _;
use serde_yaml::from_reader;

use crate::events::run as run_events;
use crate::s2s::run as run_s2s;
use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::irc::network::Network;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to config file
    #[clap(index = 1)]
    config: std::path::PathBuf,
    #[clap(long, short, action = ArgAction::Count)]
    verbose: u8,
}

trait FromFile {
    fn from_file(path: impl AsRef<Path>) -> Result<Config, ConfigError>;
}

impl FromFile for Config {
    fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let file = File::open(path).map_err(ConfigError::Io)?;
        let reader = BufReader::new(file);

        from_reader(reader).map_err(|v| ConfigError::Parse(v.into()))
    }
}

#[derive(Debug)]
enum Error {
    S2s(self::s2s::Error),
    Events(self::events::Error),
}

impl From<self::s2s::Error> for Error {
    fn from(value: self::s2s::Error) -> Self {
        Self::S2s(value)
    }
}

impl From<self::events::Error> for Error {
    fn from(value: self::events::Error) -> Self {
        Self::Events(value)
    }
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let config = Config::from_file(args.config).unwrap();

    let mut s2s_handler = haematite_s2s::ts6::Handler::try_from(config.server.clone()).unwrap();

    let events_handler = haematite_events::handler::amqp::Handler::connect(&config.amqp.address)
        .await
        .unwrap();

    let mut network = Network {
        me: config.server.clone(),
        ..Network::default()
    };

    let (tx, rx) = tokio::sync::mpsc::channel(100_000_000);

    tokio::try_join!(
        run_s2s(&config, &mut network, &mut s2s_handler, tx, args.verbose).map_err(Error::from),
        run_events(events_handler, rx).map_err(Error::from),
    )
    .unwrap();

    for (command, times) in s2s_handler.times {
        let command = format!("{command}:");
        let nanoseconds: u128 = times.iter().sum();
        let average = nanoseconds / (times.len() as u128);
        println!(
            "{command: <7}   {:0>3}.{:0>6}ms overall   {:0>2}.{:0>3}μs avg   {: >5} lines",
            nanoseconds / 1_000_000,
            nanoseconds % 1_000_000,
            average / 1_000,
            average % 1_000,
            times.len(),
        );
    }
}
