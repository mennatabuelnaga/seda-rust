use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialAnotherConfig {
    pub chain_rpc_url: Option<String>,
}

impl PartialAnotherConfig {
    fn to_config(self, cli_options: Self) -> Result<AnotherConfig> {
        todo!()
    }
}

impl Config for PartialAnotherConfig {
    fn template() -> Self {
        Self {
            chain_rpc_url: Some("https://rpc.testnet.near.org".to_string()),
        }
    }

    fn overwrite_from_env(&mut self) {
        // env_overwrite!(self.chain_rpc_url, "SEDA_CHAIN_RPC_URL");
    }
}

#[derive(Debug)]
pub struct AnotherConfig {
    pub chain_rpc_url: String,
}
