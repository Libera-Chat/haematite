#![deny(clippy::pedantic)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::equatable_if_let)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::same_name_method)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::shadow_unrelated)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::similar_names)]

mod api;
mod s2s;
mod tls;

use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, RwLock};

use clap::Parser;
use futures::future::TryFutureExt as _;
use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::network::Network;
use haematite_s2s::handler::Handler;
use haematite_s2s::ts6::TS6Handler;
use serde_yaml::from_reader;

use crate::api::run as run_api;
use crate::s2s::{run as run_s2s, Error as S2sError};

#[derive(Debug)]
enum Error {
    Api(Infallible),
    S2s(S2sError),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to config file
    #[clap(index = 1)]
    config: std::path::PathBuf,
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

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let (config, handler) = match Config::from_file(args.config) {
        Ok(it) => {
            // Assumption: this will be controlled by the config in the future.
            let handler = TS6Handler::new();
            if let Err(e) = handler.validate_config(&it) {
                eprintln!("invalid config: {}", e);
                std::process::exit(1);
            }
            (it, handler)
        }
        Err(err) => {
            eprintln!("failed to read config file: {}", err);
            std::process::exit(1);
        }
    };

    let network = Arc::new(RwLock::new(Network::new(config.server.clone())));
    tokio::try_join!(
        run_s2s(&config, Arc::clone(&network), handler).map_err(Error::S2s),
        run_api(&config, Arc::clone(&network)).map_err(Error::Api),
    )
    .unwrap();
}
