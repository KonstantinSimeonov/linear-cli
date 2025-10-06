use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Debug, Deserialize)]
pub struct LrConfig {
    pub api_key: Option<String>,
    pub default_team: Option<String>,
    pub branch_prefix: Option<String>
}

pub fn load_config() -> LrConfig {
    let mut path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".lrconfig");

    if let Ok(contents) = fs::read_to_string(&path) {
        match toml::from_str::<LrConfig>(&contents) {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("⚠️ Failed to parse config at {:?}: {}", path, err);
                LrConfig { api_key: None, default_team: None, branch_prefix: None }
            }
        }
    } else {
        LrConfig { api_key: None, default_team: None, branch_prefix: None }
    }
}
