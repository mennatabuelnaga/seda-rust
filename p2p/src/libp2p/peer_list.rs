use std::{collections::HashMap, str::FromStr};

use libp2p::{Multiaddr, PeerId};
use serde_json::Value;

pub struct PeerList {
    addr_to_peer: HashMap<Multiaddr, Option<PeerId>>,
    peer_to_addr: HashMap<PeerId, Multiaddr>,
}

impl PeerList {
    pub fn from_vec(unparsed_multi_addresses: &[String]) -> PeerList {
        let mut addr_to_peer = HashMap::new();

        unparsed_multi_addresses.iter().for_each(|unparsed_addr| {
            addr_to_peer.insert(Multiaddr::from_str(unparsed_addr).unwrap(), None);
        });

        PeerList {
            addr_to_peer,
            peer_to_addr: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, multi_addr: Multiaddr, peer_id: Option<PeerId>) {
        self.addr_to_peer.insert(multi_addr.clone(), peer_id);

        if let Some(peer) = peer_id {
            self.peer_to_addr.insert(peer, multi_addr);
        }
    }

    pub fn set_peer_id(&mut self, multi_addr: Multiaddr, peer_id: PeerId) {
        self.addr_to_peer.insert(multi_addr.clone(), Some(peer_id));
        self.peer_to_addr.insert(peer_id, multi_addr);
    }

    pub fn remove_peer_by_addr(&mut self, multi_addr: Multiaddr) {
        let item = self.addr_to_peer.get(&multi_addr);

        if let Some(Some(peer_id)) = item {
            self.peer_to_addr.remove(peer_id);
        }

        self.addr_to_peer.remove(&multi_addr);
    }

    pub fn remove_peer_by_id(&mut self, peer_id: PeerId) {
        let addr = self.peer_to_addr.get(&peer_id);

        if let Some(multi_addr) = addr {
            self.addr_to_peer.remove(multi_addr);
        }

        self.peer_to_addr.remove(&peer_id);
    }

    pub fn get_json(&self) -> Value {
        let mut result: HashMap<String, String> = HashMap::new();

        self.peer_to_addr.iter().for_each(|(peer, addr)| {
            result.insert(addr.to_string(), peer.to_base58());
        });

        serde_json::json!(result)
    }

    pub fn get_all(&self) -> HashMap<PeerId, Multiaddr> {
        self.peer_to_addr.clone()
    }
}
