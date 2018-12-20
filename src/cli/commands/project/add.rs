use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::command::Command;
use crate::config::Config;

pub struct AddCommand {}

impl Command for AddCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("add").arg(Arg::with_name("name").required(true))
    }

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
        Ok(())
    }
}
