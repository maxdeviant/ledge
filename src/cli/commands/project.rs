mod add;
mod list;
mod select;

use clap::{App, AppSettings, ArgMatches, SubCommand};

use self::add::AddCommand;
use self::list::ListCommand;
use self::select::SelectCommand;
use crate::cli::Command;
use crate::config::Config;

pub struct ProjectCommand {}

impl Command for ProjectCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("project")
            .about("Manages projects in the ledger")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(AddCommand::cli())
            .subcommand(ListCommand::cli())
            .subcommand(SelectCommand::cli())
    }

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
        match args.subcommand() {
            ("add", Some(args)) => AddCommand::exec(config, args),
            ("list", Some(args)) => ListCommand::exec(config, args),
            ("select", Some(args)) => SelectCommand::exec(config, args),
            ("", None) => Ok(()),
            _ => unreachable!(),
        }
    }
}
