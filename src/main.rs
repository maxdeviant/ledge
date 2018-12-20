#[macro_use]
extern crate clap;

mod domain;

use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use clap::{App, Arg};
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

    let matches = App::new("scribe")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or(".scribe");

    println!("{}", config);
}
