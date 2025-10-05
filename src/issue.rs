use crate::{
    client::get_client,
    exec::Execute, graphql::queries::{create_issue, teams, CreateIssue, Teams},
};
use clap::Subcommand;
use inquire::{Select, Text};

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

                let issue_description = description
                    .clone()
                    .or_else(|| Text::new("Description: ").prompt().ok())
                    .unwrap();

                let client = get_client();

                let teams_response = graphql_client::reqwest::post_graphql_blocking::<Teams, _>(
                    &client,
                    "https://api.linear.app/graphql",
                    teams::Variables {},
                )
                .unwrap();

                println!("{:?} {:?}", &teams_response.data, &teams_response.errors);

                let teams = teams_response.data.unwrap().teams.nodes;

                let team_name =
                    Select::new("Team: ", teams.iter().map(|x| x.name.clone()).collect())
                        .prompt()
                        .unwrap();

                let team_id = teams
                    .iter()
                    .find(|x| x.name == team_name)
                    .unwrap()
                    .id
                    .clone();

                let create_issue_response = graphql_client::reqwest::post_graphql_blocking::<CreateIssue, _>(
                    &client,
                    "https://api.linear.app/graphql",
                    create_issue::Variables {
                        title: issue_title,
                        description: issue_description,
                        assignee_id: None,
                        team_id: team_id,
                    },
                )
                .unwrap();

                println!("error: {:?}", create_issue_response.errors);
                println!("data: {:?}", create_issue_response.data);
            }
        }
    }
}
