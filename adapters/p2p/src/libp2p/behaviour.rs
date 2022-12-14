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
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    NetworkBehaviour,
};

use super::{super::errors::Result, P2PConfig};
use crate::P2PAdapterError;

/// Handles all P2P protocols needed for SEDA.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SedaBehaviourEvent")]
pub struct SedaBehaviour {
    /// Message propagation
    pub gossipsub: Gossipsub,
    // TODO: change discovery mechanism
    pub mdns:      Mdns,
}

impl SedaBehaviour {
    pub async fn new(_p2p_config: &P2PConfig, key_pair: &Keypair) -> Result<Self> {
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

        let topic = IdentTopic::new("test-net");
        gossipsub.subscribe(&topic)?;

        Ok(Self {
            mdns: Mdns::new(MdnsConfig::default()).await?,
            gossipsub,
        })
    }
}

pub enum SedaBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for SedaBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}

impl From<GossipsubEvent> for SedaBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        Self::Gossipsub(event)
    }
}
