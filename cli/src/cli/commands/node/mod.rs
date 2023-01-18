use clap::Subcommand;
use seda_config::AppConfig;

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
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::GetNode(get_node) => get_node.handle().await,
            Self::GetNodes(get_nodes) => get_nodes.handle().await,
            Self::RegisterNode(register_node) => register_node.handle().await,
            Self::UpdateNode(update_node) => update_node.handle().await,
            Self::UnregisterNode(unregister_node) => unregister_node.handle().await,
        }
    }
}
