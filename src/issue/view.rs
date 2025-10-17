use clap::Args;
use regex::Regex;
use termimad::crossterm::style::Stylize;

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

    let mut issue =
        gql_request::<IssueByIdentifier>(config, issue_by_identifier::Variables {
          id: issue_id,
          comment_count: if args.comments { 50 } else { 1 }
        })
            .expect("Failed to get issue")
            .issue;

    if args.web {
        webbrowser::open_browser(webbrowser::Browser::Default, issue.url.as_str())
            .expect("Failed to open browser");
        return;
    }

    render_issue(&mut issue, args);
}

fn render_issue(issue: &mut IssueByIdentifierIssue, args: &ViewIssueArgs) {
    println!(
        "Issue: [{}] {}",
        issue.identifier.clone().bold().blue(),
        issue.title.clone().bold()
    );

    println!("Url: {}", issue.url);

    if let Some(parent) = issue.parent.as_ref() {
        println!(
            "Parent: [{}] {}",
            parent.identifier.clone().yellow(),
            parent.title.clone().yellow()
        );
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
    println!("{} Description {}", "─".repeat(30), "─".repeat(30));
    let description = issue
        .description
        .as_ref()
        .map(|description| description.as_str())
        .unwrap_or("<No description>");
    termimad::print_text(&description);

    if issue.comments.nodes.len() > 0 && args.comments {
        issue
            .comments
            .nodes
            .sort_by(|a, b| a.created_at.cmp(&b.created_at));

        println!("{} Comments {}", "─".repeat(30), "─".repeat(30));
        for comment in issue.comments.nodes.iter() {
            let commenter = comment
                .user
                .as_ref()
                .map(|user| user.name.as_str())
                .unwrap_or("Unknown");
            let who = format!(
                "{} at {}:",
                commenter,
                comment.created_at.format("%d/%m/%Y")
            )
            .white()
            .on_dark_grey();
            println!("{}", who);
            termimad::print_text(&comment.body);
            println!("{}", "─".repeat(60));
        }
    }
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

    /// Whether to render comments
    #[arg(short, long)]
    comments: bool,

    /// Open in web browser
    #[arg(short, long)]
    web: bool,
}
