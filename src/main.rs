#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_from)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod database;
mod domain;
mod graphql;
mod routes;

use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};

use self::database::DatabaseConnection;
use self::domain::{
    Entry, EntryId, EntryKind, Project, ProjectHeadwayEntry, ProjectId, ProjectStatus,
};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub projects: Vec<Project>,
    pub entries: Vec<Entry>,
}

fn main() {
    dotenv().ok();

    let scribe_id = ProjectId::new();

    let log = Log {
        projects: vec![Project {
            id: scribe_id,
            name: "scribe".into(),
            status: ProjectStatus::InProgress,
            started_at: Utc.ymd(2018, 12, 16).and_hms(12, 0, 0),
        }],
        entries: vec![Entry {
            id: EntryId::new(),
            kind: EntryKind::ProjectHeadway(ProjectHeadwayEntry {
                project_id: scribe_id,
                hours_spent: 4.0,
            }),
            timestamp: Utc.ymd(2018, 12, 16).and_hms(12, 0, 0),
        }],
    };

    let mut file = File::create("stats.yaml").unwrap();
    file.write_all(&serde_yaml::to_string(&log).unwrap().into_bytes())
        .unwrap();
}
