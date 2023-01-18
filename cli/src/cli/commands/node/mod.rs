use clap::Subcommand;
use seda_config::{AppConfig, PartialChainConfigs};

use crate::Result;

mod get_node;
mod get_nodes;
mod node_result;
mod register_node;
pub use node_result::NodeResult;
mod unregister_node;
mod update_node;

#[derive(Debug, Subcommand)]
pub enum Node {
    GetNode(get_node::GetNode),
    GetNodes(get_nodes::GetNodes),
    RegisterNode(register_node::RegisterNode),
    UpdateNode(update_node::UpdateNode),
    UnregisterNode(unregister_node::UnregisterNode),
}

impl Node {
    #[tokio::main]
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        match self {
            Self::GetNode(get_node) => get_node.handle(config, chains_config).await,
            Self::GetNodes(get_nodes) => get_nodes.handle(config, chains_config).await,
            Self::RegisterNode(register_node) => register_node.handle(config, chains_config).await,
            Self::UpdateNode(update_node) => update_node.handle(config, chains_config).await,
            Self::UnregisterNode(unregister_node) => unregister_node.handle(config, chains_config).await,
        }
    }
}
