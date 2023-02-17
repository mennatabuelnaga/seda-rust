use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use crate::{merge_config_cli, Config, Result};

#[cfg(feature = "cli")]
#[derive(clap::Args)]
/// The configuration for the seda engine.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialP2PConfig {
    /// An option to override the node p2p server address config value.
    #[arg(long)]
    pub p2p_server_address: Option<String>,
    /// An option to override the node p2p known peers config value.
    #[arg(long)]
    pub p2p_known_peers:    Option<Vec<String>>,
    /// Option to use mDNS to discover peers locally
    #[arg(long)]
    pub enable_mdns:        Option<bool>,
}

#[cfg(feature = "cli")]
impl PartialP2PConfig {
    pub fn to_config(self, cli_options: Self) -> Result<P2PConfig> {
        let p2p_server_address = merge_config_cli!(
            self,
            cli_options,
            p2p_server_address,
            Ok(P2PConfigInner::P2P_SERVER_ADDRESS.to_string())
        )?;
        let p2p_known_peers = merge_config_cli!(self, cli_options, p2p_known_peers, Ok(Vec::new()))?;
        let enable_mdns = merge_config_cli!(self, cli_options, enable_mdns, Ok(true))?;

        Ok(Arc::new(P2PConfigInner {
            p2p_server_address,
            p2p_known_peers,
            enable_mdns,
        }))
    }
}

#[cfg(feature = "cli")]
impl Config for PartialP2PConfig {
    fn template() -> Self {
        Self {
            p2p_server_address: Some(P2PConfigInner::P2P_SERVER_ADDRESS.to_string()),
            p2p_known_peers:    None,
            enable_mdns:        None,
        }
    }

    fn overwrite_from_env(&mut self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfigInner {
    pub p2p_server_address: String,
    pub p2p_known_peers:    Vec<String>,
    pub enable_mdns:        bool,
}

impl P2PConfigInner {
    // TODO cfg this
    pub fn test_config() -> P2PConfig {
        Arc::new(Self {
            p2p_server_address: Self::P2P_SERVER_ADDRESS.to_string(),
            p2p_known_peers:    Vec::new(),
            enable_mdns:        true,
        })
    }

    pub fn from_json_str(s: &str) -> P2PConfig {
        let this = serde_json::from_str(s).unwrap();
        Arc::new(this)
    }
}

impl P2PConfigInner {
    pub const P2P_SERVER_ADDRESS: &str = "/ip4/0.0.0.0/tcp/0";
}

pub type P2PConfig = Arc<P2PConfigInner>;
