use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::core::{Ledger, Status};

pub fn load_status(config: &Config) -> std::io::Result<Status> {
    let mut status_file = File::open(config.root_dir.join(".status"))?;
    let mut contents = String::new();
    status_file.read_to_string(&mut contents)?;

    let status: Status = serde_json::from_str(&contents).expect("Failed to parse status file");

    Ok(status)
}

pub fn save_status(config: &Config, status: &Status) -> std::io::Result<()> {
    let mut status_file = File::create(config.root_dir.join(".status"))?;
    status_file.write_all(&serde_json::to_string_pretty(status).unwrap().into_bytes())?;

    Ok(())
}

pub fn load_ledger(config: &Config) -> std::io::Result<Ledger> {
    let mut ledger_file = File::open(config.root_dir.join(config.current_ledger.clone()))?;
    let mut contents = String::new();
    ledger_file.read_to_string(&mut contents)?;

    let ledger: Ledger = serde_json::from_str(&contents).expect("Failed to parse ledger file");

    Ok(ledger)
}

pub fn save_ledger(config: &Config, ledger: &Ledger) -> std::io::Result<()> {
    let mut ledger_file = File::create(config.root_dir.join(config.current_ledger.clone()))?;
    ledger_file.write_all(&serde_json::to_string_pretty(ledger).unwrap().into_bytes())?;

    Ok(())
}
