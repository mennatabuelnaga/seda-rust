use seda_config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    job_manager_interval_ms: u64,
}

impl Config for NodeConfig {
    // TODO
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn overwrite_from_env(&mut self) {}
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            job_manager_interval_ms: 10,
        }
    }
}
