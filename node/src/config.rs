use seda_config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    // TODO should this be overwritten from
    // env and cli?
    pub job_manager_interval_ms: u64,
    pub server_address:          Option<String>,
    pub runtime_worker_threads:  u32,
}

impl Config for NodeConfig {
    fn template() -> Self {
        Self {
            job_manager_interval_ms: 10,
            server_address:          Some("fill me in".to_string()),
            runtime_worker_threads:  2,
        }
    }

    fn overwrite_from_env(&mut self) {}
}

impl Default for NodeConfig {
    fn default() -> Self {
        let mut this = Self {
            job_manager_interval_ms: 10,
            server_address:          None,
            runtime_worker_threads:  2,
        };
        this.overwrite_from_env();
        this
    }
}
