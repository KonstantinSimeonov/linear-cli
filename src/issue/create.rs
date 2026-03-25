use std::iter;

use crate::cli_config::LrConfig;
use crate::git;
use crate::graphql::blocking_request::gql_request;
use crate::graphql::queries::{
    create_issue, issue_by_identifier, projects, search_projects, team_memberships, teams,
    CreateIssue, IssueByIdentifier, Projects, SearchProjects, TeamMemberships, Teams,
};
use clap::Args;
use inquire::{Editor, Select, Text};

pub fn issue_create(config: &LrConfig, args: &AddIssueArgs) {
    let team_id = prompt_for_team(config).unwrap();

    let issue_title = args
        .title
        .clone()
        .or_else(|| Text::new("Issue title:").prompt().ok())
        .unwrap();

    let issue_description = args
        .description
        .clone()
        .or_else(|| Editor::new("Description").prompt().ok())
        .unwrap_or_default();

    let issue_assignee = prompt_for_assignee(config, &team_id, &args.assignee);

    let parent_id = args.parent.clone().and_then(|identifier| {
        gql_request::<IssueByIdentifier>(
            config,
            issue_by_identifier::Variables {
                id: identifier.clone(),
                comment_count: 1
            },
        )
        .map(|data| data.issue.id)
        .ok()
    });

    let project_id = prompt_for_project(config, &args.project);

    let created_issue = gql_request::<CreateIssue>(
        config,
        create_issue::Variables {
            title: issue_title,
            description: issue_description,
            assignee_id: issue_assignee,
            team_id,
            parent_id,
            project_id,
        },
    )
    .expect("Failed to create issue")
    .issue_create
    .issue
    .expect("Failed to get created issue");

    println!("{}", &created_issue.url);

    if args.branch {
        let branch_name = git::get_branch_name(config, &created_issue.url, &created_issue.identifier);
        git::create_branch(&branch_name).unwrap();
    }
}

fn prompt_for_team(config: &LrConfig) -> Option<String> {
    let teams = gql_request::<Teams>(config, teams::Variables {})
        .expect("Failed to get teams")
        .teams
        .nodes;

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

const SEARCH_OPTION: &str = "Search for project...";
const NONE_OPTION: &str = "<None>";
const PROJECT_PAGE_SIZE: i64 = 50;

fn prompt_for_project(config: &LrConfig, project: &Option<String>) -> Option<String> {
    let result = gql_request::<Projects>(
        config,
        projects::Variables {
            first: Some(PROJECT_PAGE_SIZE),
            after: None,
        },
    )
    .expect("Failed to get projects")
    .projects;

    let projects = result.nodes;
    let has_more = result.page_info.has_next_page;

    if let Some(name) = project {
        return find_project_id(&projects, name)
            .or_else(|| search_project_by_name(config, name));
    }

    let mut options: Vec<String> = vec![NONE_OPTION.to_string()];
    if has_more {
        options.push(SEARCH_OPTION.to_string());
    }
    options.extend(projects.iter().map(|p| p.name.clone()));

    let selection = Select::new("Project", options).prompt().ok()?;

    match selection.as_str() {
        NONE_OPTION => None,
        SEARCH_OPTION => prompt_search_project(config),
        _ => find_project_id(&projects, &selection),
    }
}

fn find_project_id(projects: &[projects::ProjectsProjectsNodes], name: &str) -> Option<String> {
    projects.iter().find(|p| p.name == name).map(|p| p.id.clone())
}

fn search_projects(
    config: &LrConfig,
    term: &str,
) -> Vec<search_projects::SearchProjectsSearchProjectsNodes> {
    gql_request::<SearchProjects>(
        config,
        search_projects::Variables {
            term: term.to_string(),
            first: Some(PROJECT_PAGE_SIZE),
        },
    )
    .expect("Failed to search projects")
    .search_projects
    .nodes
}

fn search_project_by_name(config: &LrConfig, name: &str) -> Option<String> {
    let results = search_projects(config, name);
    results
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.id.clone())
}

fn prompt_search_project(config: &LrConfig) -> Option<String> {
    let term = Text::new("Search term:").prompt().ok()?;
    let results = search_projects(config, &term);

    if results.is_empty() {
        println!("No projects found.");
        return None;
    }

    let options: Vec<String> = iter::once(NONE_OPTION.to_string())
        .chain(results.iter().map(|p| p.name.clone()))
        .collect();

    let selection = Select::new("Project", options).prompt().ok()?;

    match selection.as_str() {
        NONE_OPTION => None,
        _ => results
            .iter()
            .find(|p| p.name == selection)
            .map(|p| p.id.clone()),
    }
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
    .expect("Failed get team memberships for assignee")
    .team
    .memberships
    .nodes;

    let issue_assignee = assignee.clone().or_else(move || {
        let assignee_options = memberships
            .iter()
            .map(|n| n.user.display_name.clone())
            .collect::<Vec<String>>();
        let assignee_name = Select::new("Assignee", assignee_options)
            .with_page_size(50)
            .prompt()
            .unwrap();
        let assignee_id = memberships
            .iter()
            .find(|n| n.user.display_name == assignee_name);

        assignee_id.map(|x| x.user.id.clone())
    });

    issue_assignee
}

#[derive(Args)]
pub struct AddIssueArgs {
    #[arg(short, long)]
    title: Option<String>,

    /// Assignee name
    #[arg(short, long)]
    assignee: Option<String>,

    #[arg(short, long)]
    description: Option<String>,

    /// Parent issue id, if any
    #[arg(short = 'p', long)]
    parent: Option<String>,

    /// Create a git branch in the format $prefix/$issue_id-$issue-slug
    #[arg(short, long)]
    branch: bool,

    /// In which project to create the issue
    #[arg(short = 'j', long)]
    project: Option<String>,
}
