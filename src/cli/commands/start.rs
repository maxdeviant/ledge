use chrono::prelude::*;
use clap::{App, ArgMatches, SubCommand};

use crate::cli::Command;
use crate::config::Config;
use crate::core::{ProjectId, Session};
use crate::util::{load_status, save_status};

pub struct StartCommand {}

impl Command for StartCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("start").about("Starts a new session")
    }

    fn exec(config: &mut Config, _args: &ArgMatches<'_>) -> Result<(), &'static str> {
        let mut status = load_status(&config).unwrap();

        status.sessions.push(Session {
            project_id: ProjectId::new(),
            started_at: Utc::now(),
        });

        save_status(&config, &status).unwrap();

        Ok(())
    }
}
