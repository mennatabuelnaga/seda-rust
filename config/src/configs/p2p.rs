use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use crate::{merge_config_cli, Config, Result};

#[cfg(feature = "cli")]
#[derive(clap::Args)]
/// The configuration for the seda engine.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialP2PConfig {
    /// The amount of inbound peers we are trying to maintain
    #[arg(long)]
    pub in_peers:               Option<i32>,
    /// The maximum amount of out peers we allow
    #[arg(long)]
    pub out_peers:              Option<i32>,
    /// An option to override the node p2p server address config value.
    #[arg(long)]
    pub p2p_server_address:     Option<String>,
    /// An option to override the node p2p known peers config value.
    #[arg(long)]
    pub p2p_known_peers:        Option<Vec<String>>,
    /// Option to use mDNS to discover peers locally
    #[arg(long)]
    pub disable_mdns:           Option<bool>,
    /// Maximum amount of peers we want to use from mDNS
    #[arg(long)]
    pub max_mdns_peers:         Option<i32>,
    /// Maximum amount of peers we want to use from our manually configured
    /// peers
    #[arg(long)]
    pub max_manual_peers:       Option<i32>,
    /// Option to disable usage of manually configured peers
    #[arg(long)]
    pub disable_manual_peers:   Option<bool>,
    /// Maximum amount of peers we fetch from using Kademlia
    #[arg(long)]
    pub max_kademlia_peers:     Option<i32>,
    /// Option to disable usage of kademlia
    #[arg(long)]
    pub disable_kademlia_peers: Option<bool>,
    /// How long a peer should not be used due a connection issue in ms
    #[arg(long)]
    pub cooldown_duration:      Option<u64>,
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
        let disable_mdns = merge_config_cli!(self, cli_options, disable_mdns, Ok(false))?;
        let max_mdns_peers = merge_config_cli!(self, cli_options, max_mdns_peers, Ok(P2PConfigInner::MAX_MDNS_PEERS))?;
        let in_peers = merge_config_cli!(self, cli_options, in_peers, Ok(P2PConfigInner::IN_PEERS))?;
        let out_peers = merge_config_cli!(self, cli_options, out_peers, Ok(P2PConfigInner::OUT_PEERS))?;
        let max_manual_peers = merge_config_cli!(
            self,
            cli_options,
            max_manual_peers,
            Ok(P2PConfigInner::MAX_MANUAL_PEERS)
        )?;
        let disable_manual_peers = merge_config_cli!(self, cli_options, disable_manual_peers, Ok(false))?;
        let disable_kademlia_peers = merge_config_cli!(self, cli_options, disable_kademlia_peers, Ok(false))?;
        let max_kademlia_peers = merge_config_cli!(
            self,
            cli_options,
            max_kademlia_peers,
            Ok(P2PConfigInner::MAX_KADEMLIA_PEERS)
        )?;

        let cooldown_duration = merge_config_cli!(
            self,
            cli_options,
            cooldown_duration,
            Ok(Duration::from_millis(P2PConfigInner::COOLDOWN_DURATION)),
            Duration::from_millis
        )?;

        Ok(Arc::new(P2PConfigInner {
            p2p_server_address,
            p2p_known_peers,
            disable_mdns,
            max_manual_peers,
            max_mdns_peers,
            disable_manual_peers,
            in_peers,
            out_peers,
            disable_kademlia_peers,
            max_kademlia_peers,
            cooldown_duration,
        }))
    }
}

#[cfg(feature = "cli")]
impl Config for PartialP2PConfig {
    fn template() -> Self {
        Self {
            p2p_server_address:     Some(P2PConfigInner::P2P_SERVER_ADDRESS.to_string()),
            p2p_known_peers:        None,
            disable_mdns:           None,
            disable_manual_peers:   None,
            max_manual_peers:       Some(P2PConfigInner::MAX_MANUAL_PEERS),
            in_peers:               Some(P2PConfigInner::IN_PEERS),
            out_peers:              Some(P2PConfigInner::OUT_PEERS),
            max_mdns_peers:         Some(P2PConfigInner::MAX_MDNS_PEERS),
            disable_kademlia_peers: None,
            max_kademlia_peers:     Some(P2PConfigInner::MAX_KADEMLIA_PEERS),
            cooldown_duration:      Some(P2PConfigInner::COOLDOWN_DURATION),
        }
    }

    fn overwrite_from_env(&mut self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfigInner {
    pub p2p_server_address:     String,
    pub p2p_known_peers:        Vec<String>,
    pub disable_mdns:           bool,
    pub max_mdns_peers:         i32,
    pub in_peers:               i32,
    pub out_peers:              i32,
    pub disable_manual_peers:   bool,
    pub max_manual_peers:       i32,
    pub max_kademlia_peers:     i32,
    pub disable_kademlia_peers: bool,
    pub cooldown_duration:      Duration,
}

impl P2PConfigInner {
    // TODO cfg this
    pub fn test_config() -> P2PConfig {
        Arc::new(Self {
            p2p_server_address:     Self::P2P_SERVER_ADDRESS.to_string(),
            p2p_known_peers:        Vec::new(),
            disable_mdns:           false,
            disable_manual_peers:   false,
            max_manual_peers:       Self::MAX_MANUAL_PEERS,
            in_peers:               Self::IN_PEERS,
            out_peers:              Self::OUT_PEERS,
            max_mdns_peers:         Self::MAX_MDNS_PEERS,
            disable_kademlia_peers: false,
            max_kademlia_peers:     Self::MAX_KADEMLIA_PEERS,
            cooldown_duration:      Duration::from_secs(Self::COOLDOWN_DURATION),
        })
    }

    pub fn from_json_str(s: &str) -> P2PConfig {
        let this = serde_json::from_str(s).unwrap();
        Arc::new(this)
    }
}

impl P2PConfigInner {
    // 30 seconds
    pub const COOLDOWN_DURATION: u64 = 30_000;
    pub const IN_PEERS: i32 = 25;
    pub const MAX_KADEMLIA_PEERS: i32 = 1000;
    pub const MAX_MANUAL_PEERS: i32 = 1000;
    pub const MAX_MDNS_PEERS: i32 = 1000;
    pub const OUT_PEERS: i32 = 100;
    pub const P2P_SERVER_ADDRESS: &str = "/ip4/0.0.0.0/tcp/0";
}

pub type P2PConfig = Arc<P2PConfigInner>;
