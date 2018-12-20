use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::core::Ledger;

pub fn load_ledger(config: &Config) -> std::io::Result<Ledger> {
    let mut ledger_file = File::open(config.root_dir.join(config.current_ledger.clone())).unwrap();
    let mut contents = String::new();
    ledger_file.read_to_string(&mut contents).unwrap();

    let ledger: Ledger = serde_json::from_str(&contents).expect("Failed to parse ledger file");

    Ok(ledger)
}

pub fn save_ledger(config: &Config, ledger: &Ledger) -> std::io::Result<()> {
    let mut ledger_file =
        File::create(config.root_dir.join(config.current_ledger.clone())).unwrap();
    ledger_file
        .write_all(&serde_json::to_string_pretty(ledger).unwrap().into_bytes())
        .unwrap();

    Ok(())
}
