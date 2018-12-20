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
    pub log_file: String,
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
        .unwrap_or(default_config_path.to_str().expect("No config file"));

    let mut config_file = File::open(config_path)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents).expect("Failed to parse config file");

    println!("config: {:?}", config);

    match matches.subcommand() {
        ("project", Some(project_matches)) => match project_matches.subcommand() {
            ("add", Some(add_matches)) => {
                let project_name = add_matches.value_of("name").expect("No project name");

                let mut log_file = File::open(config.root_dir.join(config.log_file.clone()))?;
                let mut contents = String::new();
                log_file.read_to_string(&mut contents)?;

                let mut log: Log =
                    serde_yaml::from_str(&contents).expect("Failed to parse log file");

                log.projects.push(Project {
                    id: ProjectId::new(),
                    name: project_name.to_string(),
                    status: ProjectStatus::InProgress,
                    started_at: Utc::now(),
                });

                let mut log_file = File::create(config.root_dir.join(config.log_file))?;
                log_file.write_all(&serde_yaml::to_string(&log).unwrap().into_bytes())?;
            }
            ("", None) => (),
            _ => unreachable!(),
        },
        ("", None) => (),
        _ => unreachable!(),
    }

    Ok(())
}
