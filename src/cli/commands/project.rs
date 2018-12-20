mod add;

use clap::{App, AppSettings, SubCommand};

use self::add::AddCommand;
use crate::cli::Command;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(AddCommand::cli())
}
