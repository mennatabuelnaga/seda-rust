use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;
use serde_json::json;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct UnregisterNode {
    #[arg(short, long)]
    pub node_id:     u64,
    #[command(flatten)]
    pub node_config: PartialNodeConfig,
}

impl UnregisterNode {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;

        let node_config = &config.node.to_config(self.node_config)?;
        let args = json!({
                "node_id": self.node_id.to_string(),
        })
        .to_string();
        call::<String>(
            Chain::Near,
            &node_config.contract_account_id,
            "unregister_node",
            0,
            args,
            node_config,
            &chains_config,
        )
        .await
    }
}
