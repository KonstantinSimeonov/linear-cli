use std::iter;

use crate::cli_config::LrConfig;
use crate::graphql::blocking_request::gql_request;
use crate::graphql::queries::{
    create_issue, issue_by_identifier, projects, team_memberships, teams, CreateIssue, IssueByIdentifier, Projects, TeamMemberships, Teams
};
use inquire::{Editor, Select, Text};

pub fn issue_create(
    config: &LrConfig,
    title: &Option<String>,
    assignee: &Option<String>,
    description: &Option<String>,
    parent: &Option<String>,
    project: &Option<String>,
    brach: bool
) {
    let team_id = prompt_for_team(config).unwrap();

    let issue_title = title
        .clone()
        .or_else(|| Text::new("Issue title:").prompt().ok())
        .unwrap();

    let issue_description = description
        .clone()
        .or_else(|| Editor::new("Description").prompt().ok())
        .unwrap_or_default();

    let issue_assignee = prompt_for_assignee(config, &team_id, assignee);

    let parent_id = parent.clone().and_then(|identifier| {
        gql_request::<IssueByIdentifier>(
            config,
            issue_by_identifier::Variables {
                id: identifier.clone(),
            },
        )
        .map(|data| data.issue.id)
        .ok()
    });

    let project_id = prompt_for_project(config, project);

    let create_issue_response = gql_request::<CreateIssue>(
        config,
        create_issue::Variables {
            title: issue_title,
            description: issue_description,
            assignee_id: issue_assignee,
            team_id,
            parent_id,
            project_id
        },
    )
    .unwrap();

    let created_issue = create_issue_response.issue_create.issue.unwrap();

    println!("{}", &created_issue.url);

    if brach {
      let branch_suffix = created_issue.url.split("/").last().unwrap();
      let branch_prefix = config.branch_prefix.clone().map(|prefix| format!("{}/{}", &prefix, &created_issue.identifier)).unwrap_or(created_issue.identifier.clone());
      let branch_name = format!("{}-{}", branch_prefix, branch_suffix);
      create_branch(&branch_name).unwrap();
    }
}

fn prompt_for_team(config: &LrConfig) -> Option<String> {
    let teams_response = gql_request::<Teams>(config, teams::Variables {}).unwrap();

    let teams = teams_response.teams.nodes;

    let team_name = config
        .default_team
        .clone()
        .or_else(|| {
            Select::new("Team", teams.iter().map(|x| x.name.clone()).collect())
                .prompt()
                .ok()
        })
        .unwrap();

    let team_id = teams.iter().find(|x| x.name == team_name);
    team_id.map(|t| t.id.clone())
}

fn prompt_for_project(
  config: &LrConfig,
  project: &Option<String>
  ) -> Option<String> {
  let projects_response = gql_request::<Projects>(config, projects::Variables {
    first: Some(50),
    after: None
  }).unwrap();

  let projects = projects_response.projects.nodes;

  project
    .clone()
    .or_else(|| {
      let opts: Vec<String> = iter::once("<None>".to_string()).chain(projects.iter().map(|proj| proj.name.clone())).collect();
      Select::new("Project", opts).prompt().ok()
    })
    .and_then(move |project| {
      projects.iter().find(|proj| proj.name == project).map(|proj| proj.id.clone())
    })
}

fn prompt_for_assignee(
    config: &LrConfig,
    team_id: &str,
    assignee: &Option<String>,
) -> Option<String> {
    let memberships = gql_request::<TeamMemberships>(
        config,
        team_memberships::Variables {
            team_id: team_id.to_string(),
        },
    )
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

fn create_branch(name: &str) -> std::io::Result<String> {
    std::process::Command::new("git")
        .args(["switch", "-C", name])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
}
