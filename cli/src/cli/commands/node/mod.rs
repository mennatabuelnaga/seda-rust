use clap::Subcommand;
use seda_config::{AppConfig, PartialChainConfigs};

use crate::Result;

mod get;
mod register;
mod unregister;
mod update;

#[derive(Debug, Subcommand)]
pub enum Node {
    // seda node get-node -n 1
    /// Get a node from a given node ID if it exists.
    Get(get::Node),
    // seda node get-nodes
    // seda node get-nodes -l 2 -o 1
    /// Get a list of nodes limited by the given size from an offset.
    GetNodes(get::Nodes),
    // seda node register-node -s 127.0.0.1:6666 -r 870000000000000000000
    /// Register a node from the given deposit and socket address.
    Register(register::Register),
    // seda node update-node -n 18 set-socket-address 127.0.0.1:6666
    /// Update a node by either accepting ownership, setting the pending owner,
    /// or changing the socket address.
    Update(update::Update),
    // seda node unregister-node -n 19
    /// Unregister a node from the given node ID.
    Unregister(unregister::Unregister),
}

impl Node {
    #[tokio::main]
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        match self {
            Self::Get(get_node) => get_node.handle(config, chains_config).await,
            Self::GetNodes(get_nodes) => get_nodes.handle(config, chains_config).await,
            Self::Register(register_node) => register_node.handle(config, chains_config).await,
            Self::Update(update_node) => update_node.handle(config, chains_config).await,
            Self::Unregister(unregister_node) => unregister_node.handle(config, chains_config).await,
        }
    }
}
