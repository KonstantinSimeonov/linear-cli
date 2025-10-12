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
    let r = gql_request::<MyIssues>(
        config,
        my_issues::Variables {
            first: args.count.clone().map(|c| c as i64).or(Some(20)),
            status: args.status.clone(),
        },
    );

    for node in r.unwrap().viewer.assigned_issues.nodes.iter() {
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

    #[arg(short = 'c', long = "count")]
    count: Option<usize>,
}
