mod behaviour;
pub mod peer_list;
mod transport;

#[cfg(test)]
mod libp2p_test;

use std::{str::FromStr, sync::Arc};

use async_std::io::{self, prelude::BufReadExt};
use libp2p::{
    core::ConnectedPoint,
    futures::StreamExt,
    gossipsub::{GossipsubEvent, IdentTopic},
    identity::{self},
    mdns::Event as MdnsEvent,
    swarm::{Swarm, SwarmEvent},
};
pub use libp2p::{Multiaddr, PeerId};
use parking_lot::RwLock;
use seda_runtime_sdk::p2p::{P2PCommand, P2PMessage};
use tokio::sync::mpsc::{Receiver, Sender};

use self::{behaviour::SedaBehaviour, peer_list::PeerList};
use crate::{
    errors::Result,
    libp2p::{behaviour::SedaBehaviourEvent, transport::build_tcp_transport},
};

pub const GOSSIP_TOPIC: &str = "testnet";

pub struct P2PServer {
    pub known_peers:              Arc<RwLock<PeerList>>,
    pub local_key:                identity::Keypair,
    pub server_address:           String,
    pub swarm:                    Swarm<SedaBehaviour>,
    pub message_sender_channel:   Sender<P2PMessage>,
    pub command_receiver_channel: Receiver<P2PCommand>,
}

impl P2PServer {
    pub async fn start_from_config(
        server_address: &str,
        known_peers: Arc<RwLock<PeerList>>,
        message_sender_channel: Sender<P2PMessage>,
        command_receiver_channel: Receiver<P2PCommand>,
    ) -> Result<Self> {
        // Generate Peer ID
        // TODO: Support peer id from config and storage
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        tracing::info!("Local peer id: {:?}", local_peer_id);

        // Create transport and behaviour
        let transport = build_tcp_transport(local_key.clone())?;
        let seda_behaviour = SedaBehaviour::new(&local_key).await?;

        let mut swarm = Swarm::with_threadpool_executor(transport, seda_behaviour, PeerId::from(local_key.public()));
        swarm.listen_on(server_address.parse()?)?;

        Ok(Self {
            known_peers,
            local_key,
            server_address: server_address.to_string(),
            swarm,
            message_sender_channel,
            command_receiver_channel,
        })
    }

    pub async fn dial_peers(&mut self) -> Result<()> {
        let known_peers = self.known_peers.read();

        known_peers.get_all().iter().for_each(|(peer_addr, _peer_id)| {
            match self.swarm.dial(*peer_addr) {
                Ok(_) => {
                    tracing::debug!("Dialed {}", peer_addr);
                }
                Err(error) => tracing::warn!("Couldn't dial peer ({}): {:?}", peer_addr, error),
            };
        });

        Ok(())
    }

    /// Dials the peer and adds it to our known peers list
    fn dial_peer(&mut self, peer_addr: &String) {
        let mut known_peers = self.known_peers.write();

        if let Ok(peer_multi_addr) = peer_addr.parse::<Multiaddr>() {
            match self.swarm.dial(peer_multi_addr.clone()) {
                Ok(_) => {
                    tracing::debug!("Dialed {}", peer_addr);
                    known_peers.add_peer(peer_multi_addr, None);
                }
                Err(error) => tracing::warn!("Couldn't dial peer ({}): {:?}", peer_addr, error),
            }
        } else {
            tracing::warn!("Couldn't dial peer with address: {}", peer_addr);
        }
    }

    fn add_peer(&mut self, addr: Multiaddr, peer_id: PeerId) {
        let mut known_peers = self.known_peers.write();
        known_peers.add_peer(addr, Some(peer_id));
        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
    }

    fn remove_peer(&mut self, peer_id: PeerId) {
        let mut known_peers = self.known_peers.write();
        known_peers.remove_peer_by_id(peer_id);

        if self.swarm.disconnect_peer_id(peer_id).is_ok() {
            tracing::debug!("Removed peer {peer_id}");
        }
    }

    pub async fn loop_stream(&mut self) -> Result<()> {
        // TODO: Remove stdin feature
        let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
        let topic = IdentTopic::new(GOSSIP_TOPIC);

        loop {
            tokio::select! {
                // TODO: Remove stdin feature after we got a working p2p message system
                line = stdin.select_next_some() => {
                    if let Err(e) = self.swarm
                        .behaviour_mut().gossipsub
                        .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
                        tracing::error!("Publish error: {e:?}");
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => tracing::info!("Listening on {:?}", address),
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                        for (peer_id, multiaddr) in list {
                            tracing::debug!("mDNS discovered a new peer: {}", peer_id);
                            self.add_peer(multiaddr, peer_id);
                        }
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                        let mut known_peers = self.known_peers.write();

                        if let ConnectedPoint::Dialer { address, .. } = endpoint {
                            known_peers.set_peer_id(address, peer_id)
                        }
                    }
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            tracing::debug!("mDNS discover peer has expired: {}", peer_id);
                            self.remove_peer(peer_id);
                        }
                    }
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
                    _ => {}
                },
                task = self.command_receiver_channel.recv() => match task {
                    Some(P2PCommand::Broadcast(data)) => {
                        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data) {
                            tracing::error!("Publish error: {e:?}");
                        }
                    },
                    Some(P2PCommand::Unicast(_unicast)) => {
                        unimplemented!("Todo unicast");
                    },
                    Some(P2PCommand::AddPeer(add_peer_command)) => {
                        self.dial_peer(&add_peer_command.multi_addr);
                    },
                    Some(P2PCommand::RemovePeer(remove_peer_command)) => {
                        match PeerId::from_str(&remove_peer_command.peer_id) {
                            Ok(peer_id) => self.remove_peer(peer_id),
                            Err(error) => tracing::error!("PeerId {} is invalid due: {error}", &remove_peer_command.peer_id),
                        }
                    },
                    None => {}
                },
            }
        }
    }
}
