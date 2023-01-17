use clap::Args;
use seda_config::PartialDepositAndContractID;

use crate::Result;

#[derive(Debug, Args)]
pub struct RegisterNode {
    #[arg(short, long)]
    pub socket_address: String,
    #[arg(short, long)]
    pub deposit:        String,
    #[command(flatten)]
    pub details:        PartialDepositAndContractID,
}

impl RegisterNode {
    #[tokio::main]
    pub async fn handle(self) -> Result<()> {
        todo!("chain view call");
        return Ok(());
    }
}
