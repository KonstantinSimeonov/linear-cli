use clap::Subcommand;
use colored::Colorize;

use crate::{
    cli_config::LrConfig,
    exec::Execute,
    graphql::{
        blocking_request::gql_request,
        queries::{
            projects::{self, ProjectsProjectsNodes},
            Projects,
        },
    },
};

#[derive(Subcommand)]
pub enum ProjectCommand {
    List {
        #[arg(short, long, default_missing_value = "always", default_value_t = 20)]
        count: i64,
    },
    View,
}

impl Execute for ProjectCommand {
    fn execute(&self, config: &LrConfig) {
        match self {
            ProjectCommand::View { .. } => unimplemented!(),
            ProjectCommand::List { count } => {
                let projects = gql_request::<Projects>(
                    config,
                    projects::Variables {
                        first: Some(*count),
                        after: None,
                    },
                )
                .expect("Failed to get projects")
                .projects
                .nodes;

                for project in projects.iter() {
                    render_project(project);
                }
            }
        }
    }
}

fn render_project(project: &ProjectsProjectsNodes) {
    let lead = project
        .lead
        .as_ref()
        .map(|lead| lead.name.clone())
        .unwrap_or("<None>".to_string());
    println!(
        "{} [{}] Lead: {}",
        project.name.bold().blue(),
        project.status.name,
        lead.yellow()
    )
}
