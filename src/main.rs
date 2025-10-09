mod issue;
mod exec;
mod cli;
mod graphql;
mod client;
mod cli_config;
mod project;

use crate::{cli::Cli, cli_config::load_config};
use crate::exec::Execute;
use clap::{Parser};

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let config = load_config();
    cli.command.execute(&config);
}
