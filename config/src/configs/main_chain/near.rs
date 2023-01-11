use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, merge_config_cli, Config, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Parser)]
pub struct PartialNearConfig {
    #[arg(long)]
    pub chain_rpc_url: Option<String>,
}

impl PartialNearConfig {
    pub fn to_config(self, cli_options: Self) -> Result<NearConfig> {
        let chain_rpc_url = merge_config_cli!(self, cli_options, chain_rpc_url, panic!("todo"));
        Ok(NearConfig { chain_rpc_url })
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

#[derive(Debug, Clone)]
pub struct NearConfig {
    pub chain_rpc_url: String,
}
