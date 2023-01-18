use clap::{Args, Subcommand};
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;
use serde::{ser::SerializeMap, Serialize};
use serde_json::json;

use crate::{cli::commands::call, Result};

/// Update node commands
#[derive(Clone, Debug, Subcommand)]
pub enum UpdateNodeCommand {
    AcceptOwnership,
    SetPendingOwner { owner: String },
    SetSocketAddress { address: String },
}

// Have to either manually implement Serialize or manually implement
// clap::Subcommand to get the json in the correct format.
impl Serialize for UpdateNodeCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UpdateNodeCommand::AcceptOwnership => self.serialize(serializer),
            UpdateNodeCommand::SetPendingOwner { owner } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("SetPendingOwner", owner)?;
                map.end()
            }
            UpdateNodeCommand::SetSocketAddress { address } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("SetSocketAddress", address)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct UpdateNode {
    #[arg(short, long)]
    pub node_id:     u64,
    #[command(flatten)]
    pub node_config: PartialNodeConfig,
    #[command(subcommand)]
    pub command:     UpdateNodeCommand,
}

impl UpdateNode {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;

        let node_config = &config.node.to_config(self.node_config)?;
        let args = json!({
                    "node_id": self.node_id.to_string(),
                    "command": self.command
        })
        .to_string();
        dbg!(&args);
        call::<String>(
            Chain::Near,
            &node_config.contract_account_id,
            "update_node",
            0,
            args,
            node_config,
            &chains_config,
        )
        .await
    }
}
