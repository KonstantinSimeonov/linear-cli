use clap::{Subcommand};
use inquire::{Select, Text};
use crate::exec::Execute;

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
        id: String,
    },
}

impl Execute for IssueCommand {
    fn execute(&self) {
        match self {
            IssueCommand::View { id } => println!("View: {}", id),
            IssueCommand::Add {
                title,
                assignee,
                description,
            } => {
                println!("{:?}", (&title, &assignee, &description));
                let issue_title = title
                    .clone()
                    .or_else(|| Text::new("Issue title: ").prompt().ok())
                    .unwrap();

                let v = vec!["Al Capone".to_string(), "Jabrony Williams".to_string()];
                let issue_assignee = assignee
                    .clone()
                    .or_else(|| Select::new("Assignee: ", v).prompt().ok())
                    .unwrap();

                let issue_description = description
                    .clone()
                    .or_else(|| Text::new("Description: ").prompt().ok())
                    .unwrap();

                println!(
                    "Title: {}\nAssignee: {}\nDescription: {}",
                    issue_title, issue_assignee, issue_description
                )
            }
        }
    }
}
