use clap::Args;
use seda_config::{ChainConfigs, NodeConfig, PartialDepositAndContractID};
use seda_runtime_sdk::Chain;
use serde_json::json;

use super::NodeResult;
use crate::{cli::commands::view, Result};

#[derive(Debug, Args)]
pub struct GetNode {
    #[arg(short, long)]
    pub node_id: u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
}

impl GetNode {
    pub async fn handle(self, node_config: &NodeConfig, chains_config: &ChainConfigs) -> Result<()> {
        let args = json!({
            "node_id": self.node_id.to_string(),
        })
        .to_string();
        view::<Option<NodeResult>>(
            Chain::Near,
            &node_config.contract_account_id,
            "get_node",
            args,
            chains_config,
        )
        .await
    }
}
