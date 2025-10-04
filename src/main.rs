mod issue;
mod exec;
mod cli;

use crate::cli::Cli;
use crate::exec::Execute;
use clap::{Parser};

fn main() {
    let cli = Cli::parse();
    cli.command.execute();
}
