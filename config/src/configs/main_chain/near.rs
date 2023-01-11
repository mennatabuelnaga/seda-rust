use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialNearConfig {
    pub chain_rpc_url: Option<String>,
}

impl PartialNearConfig {
    fn to_config(self, cli_options: Self) -> Result<NearConfig> {
        todo!()
    }
}

impl Config for PartialNearConfig {
    fn template() -> Self {
        Self {
            chain_rpc_url: Some("https://rpc.testnet.near.org".to_string()),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.chain_rpc_url, "SEDA_NEAR_RPC_URL");
    }
}

#[derive(Debug)]
pub struct NearConfig {
    pub chain_rpc_url: String,
}
