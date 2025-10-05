use regex::Regex;

use crate::graphql::{
    blocking_request::gql_request,
    queries::{issue_by_identifier, IssueByIdentifier},
};

pub fn issue_view(id: &Option<String>, web: bool) {
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
        gql_request::<IssueByIdentifier>(issue_by_identifier::Variables { id: issue_id }).unwrap();

    if web {
        webbrowser::open_browser(
            webbrowser::Browser::Default,
            issue_response.issue.url.as_str(),
        )
        .unwrap();
        return;
    }

    let issue = issue_response.issue;
    println!();
    println!("{} [{}]", issue.title, issue.identifier);
    println!(
        "Status: {} Assignee: {}",
        issue.state.name,
        issue
            .assignee
            .map(|assignee| assignee.name)
            .unwrap_or("<None>".to_string())
    );
    println!("{}", "─".repeat(30));
    println!(
        "{}",
        issue.description.unwrap_or("<No description>".to_string())
    );
}

fn get_branch_name() -> std::io::Result<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
}
