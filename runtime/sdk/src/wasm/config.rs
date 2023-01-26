use lazy_static::lazy_static;
use seda_config::{NodeConfig, NodeConfigInner};

use super::memory_read;

fn config() -> NodeConfig {
    let config_bytes = memory_read("*&_seda_node_config");
    NodeConfigInner::from_bytes(&config_bytes)
}

// Lazy static so its only converting from bytes once per wasm bin
lazy_static! {
    pub static ref CONFIG: NodeConfig = config();
}
