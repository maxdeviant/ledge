use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::core::{Ledger, Project, ProjectId, ProjectStatus};

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

        let mut ledger_file =
            File::open(config.root_dir.join(config.current_ledger.clone())).unwrap();
        let mut contents = String::new();
        ledger_file.read_to_string(&mut contents).unwrap();

        let mut ledger: Ledger =
            serde_json::from_str(&contents).expect("Failed to parse ledger file");

        ledger.projects.push(Project {
            id: ProjectId::new(),
            name: project_name.to_string(),
            description: None,
            status: ProjectStatus::InProgress,
            url: None,
            started_at: Utc::now(),
        });

        let mut ledger_file =
            File::create(config.root_dir.join(config.current_ledger.clone())).unwrap();
        ledger_file
            .write_all(&serde_json::to_string_pretty(&ledger).unwrap().into_bytes())
            .unwrap();

        Ok(())
    }
}
