use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::core::{Project, ProjectId, ProjectStatus};
use crate::util::{load_ledger, save_ledger};

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

        let mut ledger = load_ledger(&config).unwrap();

        ledger.projects.push(Project {
            id: ProjectId::new(),
            name: project_name.to_string(),
            description: None,
            status: ProjectStatus::InProgress,
            url: None,
            started_at: Utc::now(),
        });

        save_ledger(&config, &ledger).unwrap();

        Ok(())
    }
}
