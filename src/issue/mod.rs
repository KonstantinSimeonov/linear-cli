mod create;
mod view;
mod list;
mod edit;

use crate::cli_config::LrConfig;
use crate::issue::create::{issue_create, AddIssueArgs};
use crate::issue::list::{issue_list, ListIssueArgs};
use crate::issue::view::{issue_view, ViewIssueArgs};
use crate::issue::edit::{issue_edit, EditIssueArgs};
use crate::Execute;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum IssueCommand {
    Add(AddIssueArgs),
    View(ViewIssueArgs),
    List(ListIssueArgs),
    Edit(EditIssueArgs)
}

impl Execute for IssueCommand {
    fn execute(&self, config: &LrConfig) {
        match self {
            IssueCommand::View(args) => issue_view(config, args),
            IssueCommand::Add(args) => issue_create(config, args),
            IssueCommand::List(args) => issue_list(config, args),
            IssueCommand::Edit(args) => issue_edit(config, args)
        }
    }
}
