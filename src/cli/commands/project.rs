mod add;

use clap::{App, AppSettings, ArgMatches, SubCommand};

use self::add::AddCommand;
use crate::cli::Command;
use crate::config::Config;

pub struct ProjectCommand {}

impl Command for ProjectCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("project")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(AddCommand::cli())
    }

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
        match args.subcommand() {
            ("add", Some(args)) => AddCommand::exec(config, args),
            ("", None) => Ok(()),
            _ => unreachable!(),
        }
    }
}
