use crate::project::ProjectCommand;
use crate::{cli_config::LrConfig, issue::IssueCommand};
use crate::exec::Execute;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Cli for interacting with linear", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Issue(IssueCommand),

    #[command(subcommand)]
    Project(ProjectCommand),

    #[command(subcommand)]
    Login,
}

impl Execute for Command {
  fn execute(&self, config: &LrConfig) {
      match self {
          Command::Login => unimplemented!(),
          Command::Project(cmd) => cmd.execute(config),
          Command::Issue(cmd) => cmd.execute(config),
      }
  }
}
