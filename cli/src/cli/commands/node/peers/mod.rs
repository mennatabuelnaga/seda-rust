use clap::Subcommand;
use seda_config::AppConfig;

use crate::Result;

mod add;
mod list;
mod remove;

#[derive(Debug, Subcommand)]
pub enum Peers {
    /// Adds a peer to a running node
    Add(add::AddPeer),
    /// Lists all currently connected peers
    List(list::ListPeers),
    /// Removes a connected peer
    Remove(remove::RemovePeer),
}

impl Peers {
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::Add(add_peer) => add_peer.handle(config).await,
            Self::List(list_peers) => list_peers.handle(config).await,
            Self::Remove(remove_peer) => remove_peer.handle(config).await,
        }
    }
}
