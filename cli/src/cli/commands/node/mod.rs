use clap::Subcommand;
use seda_config::{AppConfig, PartialChainConfigs, PartialDepositAndContractID, PartialNodeConfig};

use crate::Result;

/// Update node commands
#[derive(Clone, Debug, Subcommand)]
pub enum UpdateNode {
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

#[derive(Debug, Subcommand)]
pub enum Node {
    Run {
        #[command(flatten)]
        node_config:   PartialNodeConfig,
        #[command(flatten)]
        chains_config: PartialChainConfigs,
    },
    GetNodes {
        #[arg(short, long)]
        offset:  u64,
        #[arg(short, long)]
        limit:   u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    GetNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
        #[arg(short, long)]
        deposit:        String,
        #[command(flatten)]
        details:        PartialDepositAndContractID,
    },
    UpdateNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(subcommand)]
        command: UpdateNode,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    UnregisterNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
}

impl Node {
    pub fn handle(self, config: AppConfig) -> Result<()> {
        if let Self::Run {
            node_config,
            chains_config,
        } = self
        {
            let node_config = config.node.to_config(node_config)?;
            let chains_config = config.chains.to_config(chains_config)?;
            seda_node::run(&config.seda_server_url, node_config, chains_config);

            return Ok(());
        }

        unimplemented!("")
    }
}
