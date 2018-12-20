mod commands;

use clap::{App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new("scribe")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true),
        )
        .subcommand(commands::project::cli())
}
