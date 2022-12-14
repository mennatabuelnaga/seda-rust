use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnotherConfig {
    pub chain_rpc_url: Option<String>,
}

impl Config for AnotherConfig {
    fn template() -> Self {
        Self {
            chain_rpc_url: Some("https://rpc.testnet.near.org".to_string()),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.chain_rpc_url, "CHAIN_RPC_URL");
    }
}

impl Default for AnotherConfig {
    fn default() -> Self {
        let mut this = Self { chain_rpc_url: None };
        this.overwrite_from_env();
        this
    }
}
