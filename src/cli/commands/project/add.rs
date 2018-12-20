use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::core::{Project, ProjectId, ProjectStatus};
use crate::Log;

pub struct AddCommand {}

impl Command for AddCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("add")
            .about("Adds a new project to the ledger")
            .arg(
                Arg::with_name("name")
                    .help("The name of the project")
                    .required(true),
            )
    }

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let project_name = args.value_of("name").expect("No project name");

        let mut log_file = File::open(config.root_dir.join(config.log_file.clone())).unwrap();
        let mut contents = String::new();
        log_file.read_to_string(&mut contents).unwrap();

        let mut log: Log = serde_yaml::from_str(&contents).expect("Failed to parse log file");

        log.projects.push(Project {
            id: ProjectId::new(),
            name: project_name.to_string(),
            status: ProjectStatus::InProgress,
            started_at: Utc::now(),
        });

        let mut log_file = File::create(config.root_dir.join(config.log_file.clone())).unwrap();
        log_file
            .write_all(&serde_yaml::to_string(&log).unwrap().into_bytes())
            .unwrap();

        Ok(())
    }
}
