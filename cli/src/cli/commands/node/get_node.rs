use clap::Args;
use seda_config::PartialDepositAndContractID;

use crate::{cli::commands::call, Result};

#[derive(Debug, Args)]
pub struct GetNode {
    #[arg(short, long)]
    pub node_id: u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
}

impl GetNode {
    pub async fn handle(self) -> Result<()> {
        // call::<NodeResult>(Chain::Near, config.contract_id, method_name, deposit,
        // args, config, node_config, chains_config)
        return Ok(());
    }
}
