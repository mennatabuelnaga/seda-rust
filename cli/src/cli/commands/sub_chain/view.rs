use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs};
use seda_runtime_sdk::Chain;

use crate::{cli::commands::view, Result};

#[derive(Debug, Args)]
pub struct View {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    args:        String,
}

impl View {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;
        view::<serde_json::Value>(
            self.chain,
            &self.contract_id,
            &self.method_name,
            self.args,
            &chains_config,
        )
        .await
    }
}
