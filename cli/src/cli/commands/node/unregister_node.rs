use clap::Args;
use seda_config::{ChainConfigs, NodeConfig, PartialDepositAndContractID};
use seda_runtime_sdk::Chain;
use serde_json::json;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct UnregisterNode {
    #[arg(short, long)]
    pub node_id: u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
}

impl UnregisterNode {
    pub async fn handle(self, node_config: &NodeConfig, chains_config: &ChainConfigs) -> Result<()> {
        let args = json!({
                "node_id": self.node_id.to_string(),
        })
        .to_string();
        call::<String>(
            Chain::Near,
            &node_config.contract_account_id,
            "unregister_node",
            node_config.deposit,
            args,
            node_config,
            chains_config,
        )
        .await
    }
}
