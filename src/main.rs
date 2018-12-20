#[macro_use]
extern crate clap;

mod domain;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::prelude::*;
use clap::{App, AppSettings, Arg, SubCommand};
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};

use self::domain::{
    Entry, EntryId, EntryKind, Project, ProjectHeadwayEntry, ProjectId, ProjectStatus,
};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub projects: Vec<Project>,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub root_dir: PathBuf,
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let matches = App::new("scribe")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("project")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("add").arg(Arg::with_name("name").required(true)),
                ),
        )
        .get_matches();

    let default_config_path = dirs::home_dir().unwrap().join(".scribe");

    let config_path = matches
        .value_of("config")
        .unwrap_or(default_config_path.to_str().unwrap());

    let mut config_file = File::open(config_path)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents).unwrap();

    println!("config: {:?}", config);

    match matches.subcommand() {
        ("project", Some(project_matches)) => println!("Project subcommand was used."),
        ("", None) => (),
        _ => unreachable!(),
    }

    Ok(())
}
