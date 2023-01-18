use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs};
use seda_runtime_sdk::Chain;
use serde_json::json;

use super::NodeResult;
use crate::{cli::commands::view, Result};

#[derive(Debug, Args)]
pub struct GetNode {
    #[arg(short, long)]
    pub node_id:     u64,
    #[arg(short, long)]
    pub contract_id: Option<String>,
}

impl GetNode {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;

        let contract_account_id = config.node.to_contract_account_id(self.contract_id)?;
        let args = json!({
            "node_id": self.node_id.to_string(),
        })
        .to_string();
        view::<Option<NodeResult>>(Chain::Near, &contract_account_id, "get_node", args, &chains_config).await
    }
}
