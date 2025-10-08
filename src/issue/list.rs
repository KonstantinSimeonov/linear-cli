use crate::{cli_config::LrConfig, graphql::{blocking_request::gql_request, queries::{my_issues, MyIssues}}};

pub fn issue_list(config: &LrConfig, status: &Option<String>) {
  let r = gql_request::<MyIssues>(config, my_issues::Variables {
    first: Some(20),
    status: status.clone()
  });

  for node in r.unwrap().viewer.assigned_issues.nodes.iter() {
    println!("{} {} {}", node.identifier, node.title, node.state.name)
  }
}
