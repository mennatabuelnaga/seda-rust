use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    job_manager_interval_ms: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            job_manager_interval_ms: 10,
        }
    }
}
