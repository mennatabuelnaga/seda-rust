use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    job_manager_interval: u64,
}
