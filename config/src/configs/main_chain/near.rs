use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, merge_config_cli, Config, ConfigError, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Parser)]
pub struct PartialNearConfig {
    /// An option to override the Near chain rpc url config value.
    #[arg(long)]
    pub chain_rpc_url: Option<String>,
}

impl PartialNearConfig {
    pub fn to_config(self, cli_options: Self) -> Result<NearConfig> {
        let chain_rpc_url = merge_config_cli!(
            self,
            cli_options,
            chain_rpc_url,
            Err(ConfigError::from("near.chain_rpc_url"))
        )?;
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

impl NearConfig {
    // TODO cfg this
    pub fn test_config() -> Self {
        Self {
            chain_rpc_url: "https://rpc.testnet.near.org".to_string(),
        }
    }
}
