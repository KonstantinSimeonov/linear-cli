use crate::issue::IssueCommand;
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

    Login,
}

impl Execute for Command {
  fn execute(&self) {
      match self {
          Command::Login => unimplemented!(),
          Command::Issue(cmd) => cmd.execute(),
      }
  }
}
