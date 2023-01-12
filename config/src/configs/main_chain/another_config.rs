use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialAnotherConfig {
    pub chain_rpc_url: Option<String>,
}

impl PartialAnotherConfig {
    pub fn to_config(self) -> AnotherConfig {
        // Fine cause it's just for testing.
        AnotherConfig {
            chain_rpc_url: self
                .chain_rpc_url
                .unwrap_or_else(|| "https://rpc.testnet.near.org".to_string()),
        }
    }
}

impl Config for PartialAnotherConfig {
    fn template() -> Self {
        Self {
            chain_rpc_url: Some("https://rpc.testnet.near.org".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnotherConfig {
    pub chain_rpc_url: String,
}

impl AnotherConfig {
    // TODO cfg this
    pub fn test_config() -> Self {
        Self {
            chain_rpc_url: "https://rpc.testnet.near.org".to_string(),
        }
    }
}
