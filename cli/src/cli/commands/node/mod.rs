use clap::Subcommand;
use seda_config::AppConfig;

use crate::Result;

mod get_node;
mod get_nodes;
mod register_node;
mod run;
mod unregister_node;
mod update_node;

#[derive(Debug, Subcommand)]
pub enum Node {
    GetNode(get_node::GetNode),
    GetNodes(get_nodes::GetNodes),
    RegisterNode(register_node::RegisterNode),
    Run(run::Run),
    UpdateNode(update_node::UpdateNode),
    UnregisterNode(unregister_node::UnregisterNode),
}

impl Node {
    pub fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::GetNode(get_node) => get_node.handle(),
            Self::GetNodes(get_nodes) => get_nodes.handle(),
            Self::RegisterNode(register_node) => register_node.handle(),
            Self::Run(run) => run.handle(config),
            Self::UpdateNode(update_node) => update_node.handle(),
            Self::UnregisterNode(unregister_node) => unregister_node.handle(),
        }
    }
}
