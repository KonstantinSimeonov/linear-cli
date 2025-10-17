use clap::Args;
use colored::*;
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

pub fn issue_view(config: &LrConfig, args: &ViewIssueArgs) {
    let issue_id = args.id.clone().unwrap_or_else(|| {
        let branch_name = get_branch_name().expect("Failed to get brach name");
        let name = Regex::new(r"([A-Za-z]+-\d+)")
            .expect("Invalid regex pattern")
            .captures(branch_name.as_str())
            .and_then(|capture| capture.get(1))
            .map(|capture| capture.as_str().to_string());

        name.expect(
            format!(
                "Failed to parse issue name from branch name {}",
                &branch_name
            )
            .as_str(),
        )
    });

    let issue =
        gql_request::<IssueByIdentifier>(config, issue_by_identifier::Variables { id: issue_id })
            .expect("Failed to get issue")
            .issue;

    if args.web {
        webbrowser::open_browser(webbrowser::Browser::Default, issue.url.as_str())
            .expect("Failed to open browser");
        return;
    }

    render_issue(&issue);
}

fn render_issue(issue: &IssueByIdentifierIssue) {
    println!(
        "Issue: [{}] {}",
        &issue.identifier.bold().blue(),
        &issue.title.bold()
    );

    println!("Url: {}", issue.url);

    if let Some(parent) = issue.parent.as_ref() {
      println!("Parent: [{}] {}", parent.identifier.yellow(), parent.title.yellow());
    }

    println!(
        "Status: {} Assignee: {} Created at: {}",
        &issue.state.name,
        issue
            .assignee
            .as_ref()
            .map(|assignee| assignee.name.as_str())
            .unwrap_or("<None>"),
        issue.created_at.format("%d/%m/%Y")
    );
    println!("{}", "─".repeat(30));
    let description = issue
        .description
        .as_ref()
        .map(|description| description.as_str())
        .unwrap_or("<No description>")
        .italic();
    println!("{}", description);
}

fn get_branch_name() -> std::io::Result<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
}

#[derive(Args)]
pub struct ViewIssueArgs {
    id: Option<String>,

    /// Open in web browser
    #[arg(short = 'w', long = "web")]
    web: bool,
}
