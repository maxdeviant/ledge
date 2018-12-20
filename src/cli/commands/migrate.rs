use std::fs::File;
use std::io::prelude::*;

use clap::{App, ArgMatches, SubCommand};

use crate::cli::Command;
use crate::config::Config;
use crate::core::{Ledger, LedgerId};

pub struct MigrateCommand {}

impl Command for MigrateCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("migrate").about("Migrates the ledger format")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let mut ledger_file =
            File::open(config.root_dir.join(config.current_ledger.clone())).unwrap();
        let mut contents = String::new();
        ledger_file.read_to_string(&mut contents).unwrap();

        let mut ledger: Ledger =
            serde_json::from_str(&contents).expect("Failed to parse ledger file");

        if ledger.id.is_none() {
            ledger.id = Some(LedgerId::new());
        }

        if ledger.name.is_none() {
            ledger.name = Some(String::new());
        }

        let mut ledger_file =
            File::create(config.root_dir.join(config.current_ledger.clone())).unwrap();
        ledger_file
            .write_all(&serde_json::to_string_pretty(&ledger).unwrap().into_bytes())
            .unwrap();

        Ok(())
    }
}
