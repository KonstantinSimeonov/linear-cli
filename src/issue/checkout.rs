use crate::cli_config::LrConfig;
use crate::git;
use crate::graphql::blocking_request::gql_request;
use crate::graphql::queries::{issue_by_identifier, my_issues, IssueByIdentifier, MyIssues};
use clap::Args;
use inquire::Select;

pub fn issue_checkout(config: &LrConfig, args: &CheckoutIssueArgs) {
    let (issue_id, issue_url) = args
        .id
        .clone()
        .map(|issue_id| {
            let issue = gql_request::<IssueByIdentifier>(
                config,
                issue_by_identifier::Variables {
                    id: issue_id,
                    comment_count: 1,
                },
            )
            .expect("Failed to get issue");
            (issue.issue.identifier, issue.issue.url)
        })
        .unwrap_or_else(|| {
            let issues = gql_request::<MyIssues>(
                config,
                my_issues::Variables {
                    first: Some(100),
                    status: None,
                },
            )
            .expect("Failed to get issues");

            let choice = Select::new(
                "Issue to checkout",
                issues
                    .viewer
                    .assigned_issues
                    .nodes
                    .iter()
                    .map(|issue| {
                        format!(
                            "{} {} {}",
                            issue.identifier,
                            issue.title,
                            issue.state.name
                        )
                    })
                    .collect(),
            )
            .with_page_size(20)
            .prompt()
            .ok()
            .expect("Select failed");

            let issue_id = choice.split(" ").next().expect(&format!("Split on {} failed", choice));
            let issue = issues
                .viewer
                .assigned_issues
                .nodes
                .iter()
                .find(|issue| issue.identifier == issue_id)
                .expect(&format!("Find failed with {}", issue_id));

            (issue.identifier.clone(), issue.url.clone())
        });

    let branch_name = git::get_branch_name(config, issue_url.as_str(), issue_id.as_str());
    git::create_branch(&branch_name).unwrap();
}

#[derive(Args)]
pub struct CheckoutIssueArgs {
    #[arg(short, long)]
    id: Option<String>,
}
