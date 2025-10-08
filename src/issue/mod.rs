mod create;
mod view;
mod list;

use crate::cli_config::LrConfig;
use crate::issue::create::issue_create;
use crate::issue::list::issue_list;
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
        parent: Option<String>,

        #[arg(short, long)]
        branch: bool
    },

    View {
        id: Option<String>,

        #[arg(short = 'w', long = "web")]
        web: bool,
    },

    List {
      #[arg(short, long)]
      status: Option<String>,

      #[arg(short = 'c', long = "count")]
      count: Option<usize>
    }
}

impl Execute for IssueCommand {
    fn execute(&self, config: &LrConfig) {
        match self {
            IssueCommand::View { id, web } => issue_view(config, id, *web),
            IssueCommand::Add {
                title,
                assignee,
                description,
                parent,
                branch
            } => issue_create(config, title, assignee, description, parent, *branch),
            IssueCommand::List { status, count } => issue_list(config, status, count),
        }
    }
}
