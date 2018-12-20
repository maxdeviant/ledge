use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::util::{load_ledger, load_status, save_status};

pub struct SelectCommand {}

impl Command for SelectCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("select")
            .about("Selects a project as the current project")
            .arg(
                Arg::with_name("name")
                    .help("The name of the project")
                    .required(true),
            )
    }

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let project_name = args.value_of("name").expect("No project name");

        let ledger = load_ledger(&config).unwrap();
        if let Some(project) = ledger.projects.into_iter().find(|p| p.name == project_name) {
            let mut status = load_status(&config).unwrap();

            status.current_project = Some(project.id);

            save_status(&config, &status).unwrap();
        } else {
            println!("Could not find project \"{}\"", project_name);
        }

        Ok(())
    }
}
