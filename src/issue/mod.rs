mod create;
mod view;

use crate::cli_config::LrConfig;
use crate::issue::create::issue_create;
use crate::issue::view::issue_view;
use crate::Execute;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum IssueCommand {
    Add {
        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long)]
        assignee: Option<String>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        parent: Option<String>
    },

    View {
        id: Option<String>,

        #[arg(short = 'w', long = "web")]
        web: bool,
    },
}

impl Execute for IssueCommand {
    fn execute(&self, config: &LrConfig) {
        match self {
            IssueCommand::View { id, web } => issue_view(config, id, *web),
            IssueCommand::Add {
                title,
                assignee,
                description,
                parent
            } => issue_create(config, title, assignee, description, parent),
        }
    }
}
