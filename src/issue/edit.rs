use crate::cli_config::LrConfig;
use clap::Args;

pub fn issue_edit(_config: &LrConfig, _args: &EditIssueArgs) {
  unimplemented!()
}

#[derive(Args)]
pub struct EditIssueArgs {
      #[arg(short, long)]
      status: Option<String>
}
