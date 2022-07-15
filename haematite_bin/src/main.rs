use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::path::Path;

use clap::Parser;
use haematite_models::config::Config;
use haematite_models::network::Network;
use haematite_s2s::main as s2s_main;
use serde_yaml::{from_reader, Error as YamlError};

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
    s2s_main(config, &mut network).unwrap();
}
