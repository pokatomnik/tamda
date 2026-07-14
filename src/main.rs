use clap::Parser;

use crate::util::handler::Handler;

mod cmd;
mod controllers;
mod util;

fn main() {
    let cli = cmd::cli::Cli::parse();
    let result = cli.index.handle(());
    if let Err(e) = result {
        eprintln!("{}", e.to_string())
    }
}
