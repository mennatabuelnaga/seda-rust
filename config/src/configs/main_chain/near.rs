use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearConfig {
    pub chain_rpc_url: String,
}

impl Config for NearConfig {
    fn template() -> Self {
        Self {
            chain_rpc_url: "https://rpc.testnet.near.org".to_string(),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.chain_rpc_url, "NEAR_RPC_URL");
    }
}

impl Default for NearConfig {
    fn default() -> Self {
        let mut this = Self {
            chain_rpc_url: "https://rpc.testnet.near.org".to_string(),
        };
        this.overwrite_from_env();
        this
    }
}
