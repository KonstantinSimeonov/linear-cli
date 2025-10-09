use clap::{Subcommand};
use colored::Colorize;

use crate::{cli_config::LrConfig, exec::Execute, graphql::{blocking_request::gql_request, queries::{projects, Projects}}};

#[derive(Subcommand)]
pub enum ProjectCommand {
  List,
  View
}

impl Execute for ProjectCommand {
    fn execute(&self, config: &LrConfig) {
        match self {
            ProjectCommand::View { .. } => unimplemented!(),
            ProjectCommand::List { .. } => {
              let response = gql_request::<Projects>(config, projects::Variables {
                first: Some(20),
                after: None
              }).unwrap();

              for node in response.projects.nodes.iter() {
                let lead = node.lead.as_ref().map(|lead| lead.name.clone()).unwrap_or("<None>".to_string());
                println!("{} [{}] Lead: {}", node.name.bold().blue(), node.status.name, lead.yellow())
              }
            }
        }
    }
}
