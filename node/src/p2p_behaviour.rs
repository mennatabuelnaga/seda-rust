use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SedaBehaviourEvent")]
pub struct SedaBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns:      Mdns,
}

pub enum SedaBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for SedaBehaviourEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<GossipsubEvent> for SedaBehaviourEvent {
    fn from(v: GossipsubEvent) -> Self {
        Self::Gossipsub(v)
    }
}
