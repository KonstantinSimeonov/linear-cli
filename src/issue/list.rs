use crate::{
    cli_config::LrConfig,
    graphql::{
        blocking_request::gql_request,
        queries::{my_issues, MyIssues},
    },
};
use clap::Args;
use colored::*;

pub fn issue_list(config: &LrConfig, args: &ListIssueArgs) {
    let issues = gql_request::<MyIssues>(
        config,
        my_issues::Variables {
            first: Some(args.count),
            status: args.status.clone(),
        },
    ).expect("Failed to get issues")
      .viewer
      .assigned_issues
      .nodes;

    for node in issues.iter() {
        println!(
            "[{}] {} {}",
            node.identifier.bold().blue(),
            node.title.bold(),
            node.state.name
        )
    }
}

#[derive(Args)]
pub struct ListIssueArgs {
    #[arg(short, long)]
    status: Option<String>,

    #[arg(short, long, default_missing_value = "always", default_value_t = 20)]
    count: i64,
}
