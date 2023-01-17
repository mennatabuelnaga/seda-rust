use clap::Args;
use seda_config::PartialDepositAndContractID;

use crate::Result;

#[derive(Debug, Args)]
pub struct UnregisterNode {
    #[arg(short, long)]
    pub node_id: u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
}

impl UnregisterNode {
    #[tokio::main]
    pub async fn handle(self) -> Result<()> {
        todo!("chain view call");
        return Ok(());
    }
}
