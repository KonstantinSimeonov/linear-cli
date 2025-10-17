use crate::exec::Execute;
use crate::project::ProjectCommand;
use crate::{cli_config::LrConfig, issue::IssueCommand};

use clap::{Args, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lr")]
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

    #[command(version, about = "Generate shell completion to stdout")]
    Completion(CompletionArgs),
}

#[derive(Args)]
pub struct CompletionArgs {
    #[arg(value_enum)]
    shell: clap_complete::Shell,
}

impl Execute for Command {
    fn execute(&self, config: &LrConfig) {
        match self {
            Command::Login => unimplemented!(),
            Command::Project(cmd) => cmd.execute(config),
            Command::Issue(cmd) => cmd.execute(config),
            Command::Completion(CompletionArgs { shell }) => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                clap_complete::generate(*shell, &mut cmd, bin_name, &mut std::io::stdout());
            }
        }
    }
}
