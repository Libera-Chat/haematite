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

mod s2s;
mod tls;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::Parser;
use haematite_models::config::{Config, Error as ConfigError};
use haematite_models::network::Network;
use haematite_s2s::handler::Handler;
use haematite_s2s::ts6::TS6Handler;
use serde_yaml::from_reader;

use crate::s2s::run as s2s_run;

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

fn main() {
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

    let mut network = Network::new(config.server.clone());
    s2s_run(&config, &mut network, handler).unwrap();
}
