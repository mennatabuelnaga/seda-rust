use clap::Args;
use seda_config::{ChainConfigs, PartialDepositAndContractID};
use seda_node::ChainView;
use seda_runtime_sdk::Chain;

use crate::Result;

#[derive(Debug, Args)]
pub struct View {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    args:        String,
}

impl View {
    pub async fn handle(self, chains_config: ChainConfigs) -> Result<()> {
        let view = ChainView {
            chain:       todo!(),
            contract_id: todo!(),
            method_name: todo!(),
            args:        todo!(),
            client:      todo!(),
        };
        let result = view.view().await?;
        Ok(())
    }
}
