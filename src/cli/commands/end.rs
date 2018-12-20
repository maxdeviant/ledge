use chrono::prelude::*;
use clap::{App, ArgMatches, SubCommand};

use crate::cli::Command;
use crate::config::Config;
use crate::core::{Entry, EntryId, EntryKind, ProjectHeadwayEntry};
use crate::util::{load_ledger, load_status, save_ledger, save_status};

pub struct EndCommand {}

impl Command for EndCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("end").about("Ends the current session")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let mut status = load_status(&config).unwrap();

        if let Some(session) = status.sessions.pop() {
            let mut ledger = load_ledger(&config).unwrap();

            let hours_spent = {
                let duration = Utc::now() - session.started_at;
                let minutes_spent = duration.num_minutes();

                let decimal_places = 2;
                let n = 10f64.powi(decimal_places);

                let hours = minutes_spent as f64 / 60.0;
                (hours * n).round() / n
            };

            ledger.entries.push(Entry {
                id: EntryId::new(),
                kind: EntryKind::ProjectHeadway(ProjectHeadwayEntry {
                    project_id: session.project_id,
                    hours_spent,
                }),
                timestamp: Utc::now(),
            });

            save_status(&config, &status).unwrap();
            save_ledger(&config, &ledger).unwrap();
        } else {
            println!("No current session");
        }

        Ok(())
    }
}
