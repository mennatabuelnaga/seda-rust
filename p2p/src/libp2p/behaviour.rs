use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use libp2p::{
    gossipsub::{
        Gossipsub,
        GossipsubConfigBuilder,
        GossipsubEvent,
        GossipsubMessage,
        IdentTopic,
        MessageAuthenticity,
        MessageId,
        ValidationMode,
    },
    identity::Keypair,
    kad::{store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    mdns::{self},
    swarm::NetworkBehaviour,
    PeerId,
};

use super::{super::errors::Result, GOSSIP_TOPIC};
use crate::P2PAdapterError;

/// Handles all P2P protocols needed for SEDA.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SedaBehaviourEvent")]
pub struct SedaBehaviour {
    /// Message propagation
    pub gossipsub: Gossipsub,
    // TODO: change discovery mechanism
    pub mdns:      mdns::async_io::Behaviour,

    pub kademlia: Kademlia<MemoryStore>,
}

impl SedaBehaviour {
    pub async fn new(key_pair: &Keypair) -> Result<Self> {
        let create_message_id = |message: &GossipsubMessage| {
            let mut hasher = DefaultHasher::new();
            message.data.hash(&mut hasher);
            MessageId::from(hasher.finish().to_string())
        };

        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(5))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(create_message_id)
            .build()
            .expect("Valid config");

        let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(key_pair.clone()), gossipsub_config)
            .map_err(|e| P2PAdapterError::Gossip(e.to_string()))?;

        let topic = IdentTopic::new(GOSSIP_TOPIC);
        gossipsub.subscribe(&topic)?;

        let local_peer_id = PeerId::from(key_pair.public());
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.disjoint_query_paths(true);
        kademlia_config.set_kbucket_inserts(libp2p::kad::KademliaBucketInserts::Manual);

        let kademlia_memory_store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::with_config(local_peer_id, kademlia_memory_store, kademlia_config);

        Ok(Self {
            mdns: mdns::async_io::Behaviour::new(mdns::Config::default())?,
            gossipsub,
            kademlia,
        })
    }
}

pub enum SedaBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(mdns::Event),
    Kademlia(KademliaEvent),
}

impl From<mdns::Event> for SedaBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}

impl From<GossipsubEvent> for SedaBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        Self::Gossipsub(event)
    }
}

impl From<KademliaEvent> for SedaBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}
