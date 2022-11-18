use seda_config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    job_manager_interval_ms: u64,
}

impl Config for NodeConfig {
    fn template() -> Self {
        Self {
            job_manager_interval_ms: 10,
        }
    }

    fn overwrite_from_env(&mut self) {}
}

impl Default for NodeConfig {
    fn default() -> Self {
        let mut this = Self {
            job_manager_interval_ms: 10,
        };
        this.overwrite_from_env();
        this
    }
}
