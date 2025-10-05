mod create;
mod view;

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
    },

    View {
        id: Option<String>,

        #[arg(short = 'w', long = "web")]
        web: bool,
    },
}

impl Execute for IssueCommand {
    fn execute(&self) {
        match self {
            IssueCommand::View { id, web } => issue_view(id, *web),
            IssueCommand::Add {
                title,
                assignee,
                description,
            } => issue_create(title, assignee, description),
        }
    }
}
