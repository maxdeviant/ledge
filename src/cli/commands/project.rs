mod add;

use clap::{App, AppSettings, SubCommand};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(add::cli())
}
