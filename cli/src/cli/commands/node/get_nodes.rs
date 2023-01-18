use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs};
use seda_runtime_sdk::Chain;
use serde_json::json;

use super::NodeResult;
use crate::{cli::commands::view, Result};

#[derive(Debug, Args)]
pub struct GetNodes {
    #[arg(short, long, default_value_t = 10)]
    pub limit:       u64,
    #[arg(short, long, default_value_t = 0)]
    pub offset:      u64,
    #[arg(short, long)]
    pub contract_id: Option<String>,
}

impl GetNodes {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;

        let contract_account_id = config.node.to_contract_account_id(self.contract_id)?;
        let args = json!({
                "limit": self.limit.to_string(),
                "offset": self.offset.to_string(),
        })
        .to_string();
        view::<Vec<NodeResult>>(Chain::Near, &contract_account_id, "get_nodes", args, &chains_config).await
    }
}
