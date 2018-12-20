use std::fs::File;
use std::io::prelude::*;

use clap::{App, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::Log;

pub struct ListCommand {}

impl Command for ListCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("list")
            .aliases(&["ls"])
            .about("Lists all projects in the ledger")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let mut log_file = File::open(config.root_dir.join(config.log_file.clone())).unwrap();
        let mut contents = String::new();
        log_file.read_to_string(&mut contents).unwrap();

        let log: Log = serde_json::from_str(&contents).expect("Failed to parse log file");

        println!("Projects:");

        for project in &log.projects {
            println!("  - {}", project.name);
        }

        Ok(())
    }
}
