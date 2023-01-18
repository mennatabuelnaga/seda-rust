use clap::Args;
use seda_config::{ChainConfigs, NodeConfig, PartialDepositAndContractID};
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
    pub details:        PartialDepositAndContractID,
}

impl RegisterNode {
    pub async fn handle(self, node_config: &NodeConfig, chains_config: &ChainConfigs) -> Result<()> {
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
            chains_config,
        )
        .await
    }
}
