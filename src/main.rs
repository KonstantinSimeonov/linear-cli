use clap::{Parser, Subcommand};
use inquire::{Select, Text};

fn main() {
    let cli = Cli::parse();
    cli.command.execute();
}

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Cli for interacting with linear", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand)]
    Issue(IssueCommand),

    Login,
}

#[derive(Subcommand)]
enum IssueCommand {
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

trait Execute {
    fn execute(&self);
}

impl Execute for Command {
  fn execute(&self) {
      match self {
          Command::Login => unimplemented!(),
          Command::Issue(cmd) => cmd.execute(),
      }
  }
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
