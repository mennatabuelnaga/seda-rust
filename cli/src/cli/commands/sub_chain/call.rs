use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct Call {
    chain:           Chain,
    contract_id:     String,
    method_name:     String,
    args:            String,
    call_deposit:    u128,
    #[command(flatten)]
    pub node_config: PartialNodeConfig,
}

impl Call {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let node_config = config.node.to_config(self.node_config)?;
        let chains_config = config.chains.to_config(chains_config)?;
        call::<serde_json::Value>(
            self.chain,
            &self.contract_id,
            &self.method_name,
            self.call_deposit,
            self.args,
            &node_config,
            &chains_config,
        )
        .await
    }
}
