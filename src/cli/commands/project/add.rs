use clap::{App, Arg, SubCommand};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("add").arg(Arg::with_name("name").required(true))
}
