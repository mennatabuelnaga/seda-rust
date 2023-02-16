mod behaviour;
pub mod peer_list;
mod transport;

pub mod discovery_status;
#[cfg(test)]
mod libp2p_test;

use std::{str::FromStr, sync::Arc, time::Duration};

use behaviour::SedaBehaviour;
use discovery_status::DiscoveryStatus;
use libp2p::{
    core::ConnectedPoint,
    futures::StreamExt,
    gossipsub::{GossipsubEvent, IdentTopic},
    identity::{self},
    kad::{KademliaEvent, QueryResult},
    mdns::Event as MdnsEvent,
    swarm::{DialError, NetworkBehaviour, SwarmEvent},
    Swarm,
};
pub use libp2p::{Multiaddr, PeerId};
use parking_lot::RwLock;
use peer_list::{ConnectionType, PeerInfo};
use seda_config::P2PConfig;
use seda_runtime_sdk::p2p::{P2PCommand, P2PMessage};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time,
};
use transport::build_tcp_transport;

use crate::{libp2p::behaviour::SedaBehaviourEvent, Result};

pub const GOSSIP_TOPIC: &str = "testnet";
pub const SEARCH_PEER_INTERVAL: u64 = 10_000;

pub struct P2PServer {
    swarm:            Swarm<SedaBehaviour>,
    discovery_status: Arc<RwLock<DiscoveryStatus>>,
    local_peer_id:    PeerId,

    message_sender_channel:   Sender<P2PMessage>,
    command_receiver_channel: Receiver<P2PCommand>,
}

impl P2PServer {
    pub async fn new(
        discovery_status: Arc<RwLock<DiscoveryStatus>>,
        p2p_config: P2PConfig,
        message_sender_channel: Sender<P2PMessage>,
        command_receiver_channel: Receiver<P2PCommand>,
    ) -> Result<Self> {
        // Generate Peer ID
        // TODO: Support peer id from config and storage
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        tracing::info!("Local peer id: {:?}", local_peer_id);

        let transport = build_tcp_transport(local_key.clone())?;
        let seda_behaviour = SedaBehaviour::new(&local_key).await?;
        let mut swarm = Swarm::with_threadpool_executor(transport, seda_behaviour, local_peer_id);

        swarm.listen_on(p2p_config.p2p_server_address.parse()?)?;

        Ok(Self {
            local_peer_id,
            swarm,
            discovery_status,
            command_receiver_channel,
            message_sender_channel,
        })
    }

    pub async fn start(&mut self) {
        self.search_new_peer(None);
    }

    /// Searches for new peers,
    /// goes through all the behaviours and tries to find a new peer
    /// This only tries to find one peer at a time
    pub fn search_new_peer(&mut self, skip: Option<ConnectionType>) {
        let current_discovery_method: ConnectionType = {
            let discovery_status = self.discovery_status.read();
            discovery_status.get_current_discovery_method(skip)
        };

        match current_discovery_method {
            ConnectionType::Manual => self.search_manual_peers(),
            ConnectionType::MDns => self.search_mdns_peers(),
            ConnectionType::Chain => {
                // TODO: Add chain peers
                self.search_new_peer(Some(ConnectionType::Chain));
            }
            ConnectionType::Kademlia => self.search_kademlia_peers(),
            ConnectionType::None => {
                tracing::debug!("No new peers found");
            }
        }
    }

    fn dial_peer(&mut self, addr: Multiaddr) {
        if let Err(error) = self.swarm.dial(addr.clone()) {
            tracing::warn!("Couldn't dial peer ({}): {:?}", &addr, error);
        }
    }

    fn search_manual_peers(&mut self) {
        let next_manual_peer: Option<(Multiaddr, PeerInfo)> = {
            let mut discovery_status = self.discovery_status.write();
            discovery_status.get_next_manual_peer()
        };

        if let Some((addr, _peer_info)) = next_manual_peer {
            self.dial_peer(addr);
        } else {
            self.search_new_peer(Some(ConnectionType::Manual));
        }
    }

    fn search_mdns_peers(&mut self) {
        let next_mdns_peer: Option<(Multiaddr, PeerInfo)> = {
            let mut discovery_status = self.discovery_status.write();
            discovery_status.get_next_mdns_peer()
        };

        if let Some((addr, _peer_info)) = next_mdns_peer {
            self.dial_peer(addr);
        } else {
            self.search_new_peer(Some(ConnectionType::MDns));
        }
    }

    fn search_kademlia_peers(&mut self) {
        let next_kademlia_peer: Option<(Multiaddr, PeerInfo)> = {
            let mut discovery_status = self.discovery_status.write();
            discovery_status.get_next_kademlia_peer()
        };

        if let Some((addr, _peer_info)) = next_kademlia_peer {
            self.dial_peer(addr);
        } else {
            self.swarm
                .behaviour_mut()
                .kademlia
                .get_closest_peers(self.local_peer_id);
        }
    }

    pub async fn loop_stream(&mut self) -> Result<()> {
        let topic = IdentTopic::new(GOSSIP_TOPIC);
        let mut search_peers_interval = time::interval(Duration::from_millis(SEARCH_PEER_INTERVAL));

        loop {
            tokio::select! {
                _ = search_peers_interval.tick() => {
                    self.search_new_peer(None);
                },

                event = self.swarm.select_next_some() => match event {
                    // Swarm
                    SwarmEvent::NewListenAddr { address, .. } => tracing::info!("Listening on {:?}", address),
                    SwarmEvent::ConnectionEstablished { peer_id, endpoint: ConnectedPoint::Dialer { address, .. }, .. } => {
                        tracing::debug!("Connection established with {peer_id}");

                        {
                            let mut discovery_status = self.discovery_status.write();
                            discovery_status.add_connected_peer(address.clone(), peer_id);
                        }

                        self.swarm.behaviour_mut().kademlia.add_address(&peer_id, address);
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        self.search_new_peer(None);
                    },

                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        tracing::debug!("Connection closed with {peer_id}");

                        {
                            let mut discovery_status = self.discovery_status.write();
                            discovery_status.remove_connected_peer(Some(&peer_id), None);
                        }

                        self.search_new_peer(None);
                        self.swarm.behaviour_mut().kademlia.remove_peer(&peer_id);
                    },

                    SwarmEvent::OutgoingConnectionError { peer_id: _, error } => {
                        {
                            let mut discovery_status = self.discovery_status.write();

                            if let DialError::Transport(failed_peers) = error {
                                for (addr, _transport_error) in failed_peers {
                                    discovery_status.remove_connected_peer(None, Some(&addr));
                                    discovery_status.cooldown_addr(addr);
                                }
                            }
                        }

                        self.search_new_peer(None);
                    },

                    // Gossip
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => {
                        tracing::info!(
                            "Got message: '{}' with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                        );

                        let source: Option<String> = message.source.map(|peer| peer.to_string());

                        if let Err(err) = self.message_sender_channel.send(P2PMessage { source, data: message.data }).await {
                            tracing::error!("Couldn't send message through channel: {err}");
                        }
                    },

                    // mDNS behaviour
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                        {
                            let mut discovery_status = self.discovery_status.write();

                            for (peer_id, addr) in list {
                                discovery_status.add_mdns_peer(addr, peer_id);
                            }
                        }

                        self.search_new_peer(None);
                    },

                    // Kademlia behaviour
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryProgressed {
                        result: QueryResult::GetClosestPeers(result),
                        ..
                    })) => {
                        match result {
                            Ok(closest_peers_query_result) => {
                                if !closest_peers_query_result.peers.is_empty() {
                                    let mut discovery_status = self.discovery_status.write();

                                    for peer_id in closest_peers_query_result.peers {
                                        let addresses = self.swarm.behaviour_mut().kademlia.addresses_of_peer(&peer_id);

                                        if let Some(first_address) = addresses.first() {
                                            discovery_status.add_kademlia_peer(first_address.clone(), peer_id);
                                        }
                                    }
                                } else {
                                    tracing::debug!("Kademlia couldn't find new peers");
                                }
                            }
                            Err(err) => tracing::error!("Error while using Kademlia: {err}")
                        }
                    },
                    _ => {}
                },

                task = self.command_receiver_channel.recv() => match task {
                    None => {},
                    Some(P2PCommand::Broadcast(data)) => {
                        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data) {
                            tracing::error!("Publish error: {e:?}");
                        }
                    },
                    Some(P2PCommand::Unicast(_unicast)) => {
                        unimplemented!("Todo unicast");
                    },
                    Some(P2PCommand::AddPeer(add_peer_command)) => {
                        if let Ok(multi_addr) = add_peer_command.multi_addr.parse::<Multiaddr>() {
                            {
                                let mut discovery_status = self.discovery_status.write();
                                discovery_status.add_manual_peer(multi_addr, None);
                            }

                            self.search_new_peer(None);
                        } else {
                            tracing::warn!("Couldn't add peer, invalid address: {}", add_peer_command.multi_addr);
                        }
                    },
                    Some(P2PCommand::RemovePeer(remove_peer_command)) => {
                        match PeerId::from_str(&remove_peer_command.peer_id) {
                            Ok(peer_id) => {
                                self.swarm.disconnect_peer_id(peer_id).ok();

                                {
                                    let mut discovery_status = self.discovery_status.write();
                                    discovery_status.remove_connected_peer(Some(&peer_id), None);
                                }

                                self.search_new_peer(None);
                            },
                            Err(error) => tracing::warn!("PeerId {} is invalid: {error}", &remove_peer_command.peer_id)
                        }
                    },
                    Some(P2PCommand::DiscoverPeers) => {
                        self.search_new_peer(None);
                    }
                }
            }
        }
    }
}
