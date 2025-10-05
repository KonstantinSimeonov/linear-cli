use std::env;

use crate::{
    exec::Execute,
    graphql::{
        blocking_request::gql_request,
        queries::{create_issue, team_memberships, teams, CreateIssue, TeamMemberships, Teams},
    },
};
use clap::{builder::OsStr, Subcommand};
use inquire::{Select, Text, Editor};

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
                let team_id = prompt_for_team().unwrap();

                let issue_title = title
                    .clone()
                    .or_else(|| Text::new("Issue title:").prompt().ok())
                    .unwrap();

                let issue_description = description
                    .clone()
                    .or_else(|| Editor::new("Description").prompt().ok())
                    .unwrap_or_default();

                let issue_assignee = prompt_for_assignee(&team_id, assignee);

                let create_issue_response = gql_request::<CreateIssue>(create_issue::Variables {
                    title: issue_title,
                    description: issue_description,
                    assignee_id: issue_assignee,
                    team_id: team_id,
                })
                .unwrap();

                println!("{}", create_issue_response.issue_create.issue.unwrap().url);
            }
        }
    }
}

fn prompt_for_team() -> Option<String> {
    let teams_response = gql_request::<Teams>(teams::Variables {}).unwrap();

    let teams = teams_response.teams.nodes;

    let team_name = Select::new("Team", teams.iter().map(|x| x.name.clone()).collect())
        .prompt()
        .unwrap();

    let team_id = teams.iter().find(|x| x.name == team_name);
    team_id.map(|t| t.id.clone())
}

fn prompt_for_assignee(team_id: &str, assignee: &Option<String>) -> Option<String> {
    let memberships = gql_request::<TeamMemberships>(team_memberships::Variables {
        team_id: team_id.to_string(),
    })
    .unwrap();

    let issue_assignee = assignee.clone().or_else(move || {
        let m = memberships
            .team
            .memberships
            .nodes
            .iter()
            .map(|n| n.user.display_name.clone())
            .collect::<Vec<String>>();
        let assignee_name = Select::new("Assignee", m)
            .with_page_size(50)
            .prompt()
            .unwrap();
        let assignee_id = memberships
            .team
            .memberships
            .nodes
            .iter()
            .find(|n| n.user.display_name == assignee_name);

        assignee_id.map(|x| x.user.id.clone())
    });

    issue_assignee
}
