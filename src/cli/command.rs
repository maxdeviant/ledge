use clap::{App, ArgMatches};

use crate::config::Config;

pub trait Command {
    fn cli() -> App<'static, 'static>;

    fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str>;
}
