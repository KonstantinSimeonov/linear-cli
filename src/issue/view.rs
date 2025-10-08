use regex::Regex;

use crate::{
    cli_config::LrConfig,
    graphql::{
        blocking_request::gql_request,
        queries::{
            issue_by_identifier::{self, IssueByIdentifierIssue},
            IssueByIdentifier,
        },
    },
};

pub fn issue_view(config: &LrConfig, id: &Option<String>, web: bool) {
    let issue_id = id
        .clone()
        .or_else(|| {
            let branch_name = get_branch_name().unwrap();
            let name = Regex::new(r"([A-Za-z]+-\d+)")
                .unwrap()
                .captures(branch_name.as_str())
                .and_then(|capture| capture.get(1))
                .map(|capture| capture.as_str().to_string());

            name
        })
        .unwrap();

    let issue_response =
        gql_request::<IssueByIdentifier>(config, issue_by_identifier::Variables { id: issue_id })
            .unwrap();

    if web {
        webbrowser::open_browser(
            webbrowser::Browser::Default,
            issue_response.issue.url.as_str(),
        )
        .unwrap();
        return;
    }

    render_issue(&issue_response.issue);
}

fn render_issue(issue: &IssueByIdentifierIssue) {
    println!();
    println!("{} [{}]", &issue.title, &issue.identifier);
    println!(
        "Status: {} Assignee: {}",
        &issue.state.name,
        issue
            .assignee
            .as_ref()
            .map(|assignee| assignee.name.as_str())
            .unwrap_or("<None>")
    );
    println!("{}", "─".repeat(30));
    let description = issue
        .description
        .as_ref()
        .map(|description| description.as_str())
        .unwrap_or("<No description>");
    println!("{}", description);
}

fn get_branch_name() -> std::io::Result<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
}
