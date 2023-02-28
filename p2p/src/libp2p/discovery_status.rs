use std::{collections::HashMap, sync::Arc, time::SystemTime};

use libp2p::{Multiaddr, PeerId};
use parking_lot::RwLock;
use seda_config::P2PConfig;

use super::peer_list::{ConnectionType, PeerInfo};
use crate::PeerList;

/// The manager should use this
/// The manager should not check the config but just have a switch statement
/// This struct will check all connections and config which discovery method
/// should be used

pub struct DiscoveryStatusInner {
    p2p_config: P2PConfig,

    pub connected_peers: PeerList,

    /// Peers who should not be connected to until the cooldown period has ended
    /// This prevents the manager to try to connect to the same failing peer
    cooldown_addrs: HashMap<Multiaddr, SystemTime>,

    // Peers that have been found by a discovery mechanism
    // but don't have to be connected, we can use this if we want to add them
    found_manual_peers:   PeerList,
    found_chain_peers:    PeerList,
    found_mdns_peers:     PeerList,
    found_kademlia_peers: PeerList,
}

impl DiscoveryStatusInner {
    pub fn new(p2p_config: P2PConfig, inital_manual_peers: PeerList) -> Self {
        Self {
            p2p_config,
            found_manual_peers: inital_manual_peers,
            found_chain_peers: PeerList::default(),
            found_mdns_peers: PeerList::default(),
            found_kademlia_peers: PeerList::default(),
            connected_peers: PeerList::default(),
            cooldown_addrs: HashMap::default(),
        }
    }

    fn get_connected_len_by_type(&self, connection_type: ConnectionType) -> usize {
        self.connected_peers
            .get_all_info()
            .iter()
            .filter(|(_addr, info)| info.conn_type == connection_type)
            .count()
    }

    pub fn get_connected_list(&self) -> PeerList {
        self.connected_peers.clone()
    }

    /// Checks if the peer is already connected to and hasn't been tried
    /// recently
    fn is_unused_addr(&mut self, addr: &Multiaddr) -> bool {
        if self.connected_peers.has_addr(addr) {
            return false;
        }

        if let Some(peer_cooldowned_time) = self.cooldown_addrs.get(addr) {
            let now = SystemTime::now();

            // When the cooldown period has exceeded
            if now > (*peer_cooldowned_time + self.p2p_config.cooldown_duration) {
                self.cooldown_addrs.remove(addr);
                return true;
            }

            return false;
        }

        true
    }

    /// Gets the discovery method the manager should use
    /// Based on priority and maximum allowed peers from that specific discovery
    /// method
    /// * `skip` - Allows to skip any higher or equal priority then the one
    ///   given (if for example we already exhausted that source)
    pub fn get_current_discovery_method(&self, skip: Option<ConnectionType>) -> ConnectionType {
        // We already reached the maximum required peers, we don't need more
        if self.connected_peers.len() as i32 >= self.p2p_config.out_peers {
            return ConnectionType::None;
        }

        let skip = skip.unwrap_or(ConnectionType::None);

        if !self.p2p_config.disable_manual_peers
            && self.get_connected_len_by_type(ConnectionType::Manual) < (self.p2p_config.max_manual_peers as usize)
            && skip < ConnectionType::Manual
        {
            return ConnectionType::Manual;
        }

        if !self.p2p_config.disable_mdns
            && self.get_connected_len_by_type(ConnectionType::MDns) < (self.p2p_config.max_mdns_peers as usize)
            && skip < ConnectionType::MDns
        {
            return ConnectionType::MDns;
        }

        if !self.p2p_config.disable_kademlia_peers
            && self.get_connected_len_by_type(ConnectionType::Kademlia) < (self.p2p_config.max_kademlia_peers as usize)
            && skip < ConnectionType::Kademlia
        {
            return ConnectionType::Kademlia;
        }

        ConnectionType::None
    }

    pub fn cooldown_addr(&mut self, addr: Multiaddr) {
        println!("Added {addr} to cooldown");
        self.cooldown_addrs.insert(addr, SystemTime::now());
    }

    pub fn add_connected_peer(&mut self, addr: Multiaddr, peer_id: PeerId) {
        self.connected_peers.set_peer_id(addr, peer_id);
    }

    pub fn remove_connected_peer(&mut self, peer_id: Option<&PeerId>, address: Option<&Multiaddr>) {
        if let Some(peer_id) = peer_id {
            if let Some((_addr, peer_info)) = self.connected_peers.get_peer_by_id(peer_id) {
                match peer_info.conn_type {
                    ConnectionType::Chain => self.found_chain_peers.remove_peer_by_id(peer_id),
                    ConnectionType::Kademlia => self.found_kademlia_peers.remove_peer_by_id(peer_id),
                    ConnectionType::MDns => self.found_mdns_peers.remove_peer_by_id(peer_id),
                    // Manual peers (and none) should not remove anything
                    // We are expecting them to maybe be available again
                    _ => {}
                }
            }

            self.connected_peers.remove_peer_by_id(peer_id);
        }

        if let Some(addr) = address {
            if let Some(peer_info) = self.connected_peers.get_peer_by_addr(addr) {
                match peer_info.conn_type {
                    ConnectionType::Chain => self.found_chain_peers.remove_peer_by_addr(addr),
                    ConnectionType::Kademlia => self.found_kademlia_peers.remove_peer_by_addr(addr),
                    ConnectionType::MDns => self.found_mdns_peers.remove_peer_by_addr(addr),
                    // Manual peers (and none) should not remove anything
                    // We are expecting them to maybe be available again
                    _ => {}
                }
            }

            self.connected_peers.remove_peer_by_addr(addr);
        }
    }

    pub fn get_next_manual_peer(&mut self) -> Option<(Multiaddr, PeerInfo)> {
        for (addr, peer_info) in self.found_manual_peers.get_all_info().iter() {
            if self.is_unused_addr(addr) {
                self.connected_peers
                    .add_peer(addr.clone(), peer_info.peer_id, ConnectionType::Manual);

                return Some((addr.clone(), peer_info.clone()));
            }
        }

        None
    }

    pub fn add_mdns_peer(&mut self, addr: Multiaddr, peer_id: PeerId) {
        self.found_mdns_peers
            .add_peer(addr, Some(peer_id), ConnectionType::MDns);
    }

    pub fn get_next_mdns_peer(&mut self) -> Option<(Multiaddr, PeerInfo)> {
        for (addr, peer_info) in self.found_mdns_peers.get_all_info().iter() {
            if self.is_unused_addr(addr) {
                self.connected_peers
                    .add_peer(addr.clone(), peer_info.peer_id, ConnectionType::MDns);

                return Some((addr.clone(), peer_info.clone()));
            }
        }

        None
    }

    pub fn add_kademlia_peer(&mut self, addr: Multiaddr, peer_id: PeerId) {
        self.found_kademlia_peers
            .add_peer(addr, Some(peer_id), ConnectionType::Kademlia);
    }

    pub fn get_next_kademlia_peer(&mut self) -> Option<(Multiaddr, PeerInfo)> {
        for (addr, peer_info) in self.found_kademlia_peers.get_all_info().iter() {
            if self.is_unused_addr(addr) {
                self.connected_peers
                    .add_peer(addr.clone(), peer_info.peer_id, ConnectionType::Kademlia);

                return Some((addr.clone(), peer_info.clone()));
            }
        }

        None
    }

    pub fn add_manual_peer(&mut self, addr: Multiaddr, peer_id: Option<PeerId>) {
        self.found_kademlia_peers
            .add_peer(addr, peer_id, ConnectionType::Kademlia);
    }
}

pub type DiscoveryStatus = Arc<RwLock<DiscoveryStatusInner>>;
