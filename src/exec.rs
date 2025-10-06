use crate::cli_config::LrConfig;

pub trait Execute {
    fn execute(&self, config: &LrConfig);
}
