mod domain;

use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};

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
    let hitheryon_id = ProjectId::new();

    let log = Log {
        projects: vec![
            Project {
                id: scribe_id,
                name: "scribe".into(),
                status: ProjectStatus::InProgress,
                started_at: Utc.ymd(2018, 12, 16).and_hms(12, 0, 0),
            },
            Project {
                id: hitheryon_id,
                name: "Hitheryon".into(),
                status: ProjectStatus::InProgress,
                started_at: Utc.ymd(2018, 8, 25).and_hms(15, 34, 27),
            },
        ],
        entries: vec![
            Entry {
                id: EntryId::new(),
                kind: EntryKind::ProjectHeadway(ProjectHeadwayEntry {
                    project_id: scribe_id,
                    hours_spent: 4.0,
                }),
                timestamp: Utc.ymd(2018, 12, 16).and_hms(12, 0, 0),
            },
            Entry {
                id: EntryId::new(),
                kind: EntryKind::ProjectHeadway(ProjectHeadwayEntry {
                    project_id: hitheryon_id,
                    hours_spent: 10.0,
                }),
                timestamp: Utc.ymd(2018, 8, 27).and_hms(3, 41, 29),
            },
            Entry {
                id: EntryId::new(),
                kind: EntryKind::ProjectHeadway(ProjectHeadwayEntry {
                    project_id: hitheryon_id,
                    hours_spent: 1.5,
                }),
                timestamp: Utc.ymd(2018, 8, 25).and_hms(16, 56, 41),
            },
        ],
    };

    let mut file = File::create("stats.yaml").unwrap();
    file.write_all(&serde_yaml::to_string(&log).unwrap().into_bytes())
        .unwrap();
}
