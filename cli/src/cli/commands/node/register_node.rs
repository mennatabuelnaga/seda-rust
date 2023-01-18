use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;
use serde_json::json;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct RegisterNode {
    #[arg(short, long)]
    pub deposit:        u128,
    #[arg(short, long)]
    pub socket_address: String,
    #[command(flatten)]
    pub node_config:    PartialNodeConfig,
}

impl RegisterNode {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;

        let node_config = &config.node.to_config(self.node_config)?;
        let args = json!({
                "socket_address": self.socket_address,
        })
        .to_string();
        call::<String>(
            Chain::Near,
            &node_config.contract_account_id,
            "register_node",
            self.deposit,
            args,
            node_config,
            &chains_config,
        )
        .await
    }
}
