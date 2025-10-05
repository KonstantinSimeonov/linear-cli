mod issue;
mod exec;
mod cli;
mod graphql;
mod client;

use crate::cli::Cli;
use crate::exec::Execute;
use clap::{Parser};

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    cli.command.execute();
}
