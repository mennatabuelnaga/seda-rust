use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct Call {
    /// The sub-chain to call.
    chain:           Chain,
    /// The contract ID for the sub-chain.
    contract_id:     String,
    /// The method name to call.
    method_name:     String,
    /// The args to pass to the call method.
    args:            String,
    /// The deposit for the call method.
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
