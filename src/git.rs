use crate::cli_config::LrConfig;


pub fn create_branch(name: &str) -> std::io::Result<String> {
    std::process::Command::new("git")
        .args(["switch", "-C", name])
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
}

pub fn get_branch_name(config: &LrConfig, issue_url: &str, issue_id: &str) -> String {
    let branch_suffix = 
        issue_url
        .split("/")
        .last()
        .expect(format!("Could not get branch name from {}", issue_url).as_str());

    let branch_prefix = config
        .branch_prefix
        .clone()
        .map(|prefix| format!("{}/{}", &prefix, issue_id))
        .unwrap_or(issue_id.to_string());
    let branch_name = format!("{}-{}", branch_prefix, branch_suffix);

    branch_name
}
