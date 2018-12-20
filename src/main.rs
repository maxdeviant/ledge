#[macro_use]
extern crate clap;

mod cli;
mod config;
mod core;
mod util;

use std::fs::File;
use std::io::prelude::*;

use dotenv::dotenv;

use crate::config::Config;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let matches = cli::app().get_matches();

    let default_config_path = dirs::home_dir().unwrap().join(".ledge");

    let config_path = matches
        .value_of("config")
        .unwrap_or(default_config_path.to_str().expect("No config file"));

    let mut config_file = File::open(config_path)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    let mut config: Config = toml::from_str(&contents).expect("Failed to parse config file");

    cli::exec(&mut config, &matches).unwrap();

    Ok(())
}
