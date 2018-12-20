use std::fs::File;
use std::io::prelude::*;

use clap::{App, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;
use crate::core::Ledger;
use crate::util::load_ledger;

pub struct ListCommand {}

impl Command for ListCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("list")
            .aliases(&["ls"])
            .about("Lists all projects in the ledger")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let mut ledger_file =
            File::open(config.root_dir.join(config.current_ledger.clone())).unwrap();
        let mut contents = String::new();
        ledger_file.read_to_string(&mut contents).unwrap();

        let ledger: Ledger = load_ledger(&config).unwrap();

        println!("Projects:");

        for project in &ledger.projects {
            println!("  - {}", project.name);
        }

        Ok(())
    }
}
