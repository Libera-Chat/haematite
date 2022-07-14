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
use std::io::{BufReader, Error as IoError};
use std::path::Path;

use clap::Parser;
use haematite_models::config::Config;
use haematite_models::network::Network;
use haematite_s2s::ts6::TS6Handler;
use serde_yaml::{from_reader, Error as YamlError};

use crate::s2s::run as s2s_run;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to config file
    #[clap(index = 1)]
    config: std::path::PathBuf,
}

#[derive(Debug)]
pub enum Error {
    ConfigIo(IoError),
    ConfigParse(YamlError),
}

trait FromFile {
    fn from_file(path: impl AsRef<Path>) -> Result<Config, Error>;
}

impl FromFile for Config {
    fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::ConfigIo)?;
        let reader = BufReader::new(file);

        from_reader(reader).map_err(Error::ConfigParse)
    }
}

fn main() {
    let args = CliArgs::parse();

    let config = match Config::from_file(args.config) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("failed to read config file: {:?}", err);
            std::process::exit(1);
        }
    };

    let mut network = Network::new(config.server.clone());
    let handler = TS6Handler::new();
    s2s_run(config, &mut network, handler).unwrap();
}
