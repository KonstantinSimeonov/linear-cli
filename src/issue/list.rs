use crate::{cli_config::LrConfig, graphql::{blocking_request::gql_request, queries::{my_issues, MyIssues}}};
use colored::*;

pub fn issue_list(config: &LrConfig, status: &Option<String>, count: &Option<usize>) {
  let r = gql_request::<MyIssues>(config, my_issues::Variables {
    first: count.clone().map(|c| c as i64).or(Some(20)),
    status: status.clone()
  });

  for node in r.unwrap().viewer.assigned_issues.nodes.iter() {
    println!("[{}] {} {}", node.identifier.bold().blue(), node.title.bold(), node.state.name)
  }
}
