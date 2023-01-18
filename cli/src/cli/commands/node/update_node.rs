use clap::{Args, Subcommand};
use seda_config::PartialDepositAndContractID;

use crate::Result;

/// Update node commands
#[derive(Clone, Debug, Subcommand)]
pub enum UpdateNodeCommand {
    AcceptOwnership,
    SetPendingOwner {
        #[arg(short, long)]
        owner: String,
    },
    SetSocketAddress {
        #[arg(short, long)]
        address: String,
    },
}

#[derive(Debug, Args)]
pub struct UpdateNode {
    #[arg(short, long)]
    pub node_id: u64,
    #[command(flatten)]
    pub details: PartialDepositAndContractID,
    #[command(subcommand)]
    pub command: UpdateNodeCommand,
}

impl UpdateNode {
    pub async fn handle(self) -> Result<()> {
        todo!("chain view call");
        return Ok(());
    }
}
