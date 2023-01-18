use clap::Args;
use seda_config::{ChainConfigs, NodeConfig, PartialDepositAndContractID};
use seda_runtime_sdk::Chain;
use serde_json::json;

use super::NodeResult;
use crate::{cli::commands::view, Result};

#[derive(Debug, Args)]
pub struct GetNodes {
    #[arg(short, long, default_value_t = 10)]
    pub limit:   u64,
    #[arg(short, long, default_value_t = 0)]
    pub offset:  u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
}

impl GetNodes {
    pub async fn handle(self, node_config: &NodeConfig, chains_config: &ChainConfigs) -> Result<()> {
        let args = json!({
                "limit": self.limit.to_string(),
                "offset": self.offset.to_string(),
        })
        .to_string();
        view::<Vec<NodeResult>>(
            Chain::Near,
            &node_config.contract_account_id,
            "get_nodes",
            args,
            chains_config,
        )
        .await
    }
}
