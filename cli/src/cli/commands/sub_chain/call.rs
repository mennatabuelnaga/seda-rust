use clap::Args;
use seda_config::PartialDepositAndContractID;
use seda_runtime_sdk::Chain;

use crate::Result;

#[derive(Debug, Args)]
pub struct Call {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    args:        String,
    deposit:     String,
}

impl Call {
    pub async fn handle(self) -> Result<()> {
        todo!("chain view call");
        return Ok(());
    }
}
