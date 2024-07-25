use clap::{Arg, Command};

pub fn stock_command() -> Command {
    Command::new("stock").arg(Arg::new("symbol").required(true).long("symbol").short('s'))
}
