use clap::{Args, Subcommand};
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};
use seda_runtime_sdk::Chain;
use serde::{ser::SerializeMap, Serialize};
use serde_json::json;

use crate::{cli::commands::call, Result};

/// Update node commands
#[derive(Clone, Debug, Subcommand)]
pub enum UpdateCommand {
    AcceptOwnership,
    SetPendingOwner { owner: String },
    SetSocketAddress { address: String },
}

// Have to either manually implement Serialize or manually implement
// clap::Subcommand to get the json in the correct format.
impl Serialize for UpdateCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::AcceptOwnership => self.serialize(serializer),
            Self::SetPendingOwner { owner } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("SetPendingOwner", owner)?;
                map.end()
            }
            Self::SetSocketAddress { address } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("SetSocketAddress", address)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct Update {
    #[arg(short, long)]
    pub node_id:     u64,
    #[command(flatten)]
    pub node_config: PartialNodeConfig,
    #[command(subcommand)]
    pub command:     UpdateCommand,
}

impl Update {
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        let chains_config = config.chains.to_config(chains_config)?;
        let node_config = &config.node.to_config(self.node_config)?;
        let args = json!({
                    "node_id": self.node_id.to_string(),
                    "command": self.command
        })
        .to_string();
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
