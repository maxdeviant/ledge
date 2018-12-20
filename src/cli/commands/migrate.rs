use clap::{App, ArgMatches, SubCommand};

use crate::cli::Command;
use crate::config::Config;
use crate::util::{load_ledger, save_ledger};

pub struct MigrateCommand {}

impl Command for MigrateCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("migrate").about("Migrates the ledger format")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let ledger = load_ledger(&config).unwrap();
        save_ledger(&config, &ledger).unwrap();

        Ok(())
    }
}
