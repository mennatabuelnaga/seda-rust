use clap::Subcommand;
use seda_config::AppConfig;

use crate::Result;

mod add;
mod list;

#[derive(Debug, Subcommand)]
pub enum Peers {
    /// Adds a peer to a running node
    Add(add::AddPeer),
    List(list::ListPeers),
}

impl Peers {
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::Add(add_peer) => add_peer.handle(config).await,
            Self::List(list_peers) => list_peers.handle(config).await,
        }
    }
}
