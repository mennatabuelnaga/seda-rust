mod behaviour;
mod transport;

#[cfg(test)]
mod libp2p_test;

use async_std::io::{self, prelude::BufReadExt};
use libp2p::{
    futures::{select, StreamExt},
    gossipsub::{GossipsubEvent, IdentTopic},
    identity::{self},
    mdns::MdnsEvent,
    swarm::{Swarm, SwarmEvent},
    Multiaddr,
    PeerId,
};

use self::behaviour::SedaBehaviour;
use crate::{
    errors::Result,
    libp2p::{behaviour::SedaBehaviourEvent, transport::build_tcp_transport},
};

pub struct P2PConfig {
    pub server_address: String,
    pub known_peers:    Vec<String>,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            server_address: "/ip4/0.0.0.0/tcp/0".to_string(),
            known_peers:    Default::default(),
        }
    }
}

pub struct P2PServer {
    pub config:    P2PConfig,
    pub local_key: identity::Keypair,
    pub swarm:     Swarm<SedaBehaviour>,
}

impl P2PServer {
    pub async fn start_from_config(config: P2PConfig) -> Result<Self> {
        // Generate Peer ID
        // TODO: Support peer id from config and storage
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        tracing::info!("Local peer id: {:?}", local_peer_id);

        // Create transport and behaviour
        let transport = build_tcp_transport(local_key.clone())?;
        let seda_behaviour = SedaBehaviour::new(&config, &local_key).await?;

        let mut swarm = Swarm::new(transport, seda_behaviour, PeerId::from(local_key.public()));
        swarm.listen_on(config.server_address.parse()?)?;

        Ok(Self {
            config,
            local_key,
            swarm,
        })
    }

    pub async fn dial_peers(&mut self) -> Result<()> {
        self.config.known_peers.iter().for_each(|peer_addr| {
            if let Ok(remote) = peer_addr.parse::<Multiaddr>() {
                match self.swarm.dial(remote) {
                    Ok(_) => {
                        tracing::debug!("Dialed {}", peer_addr);
                    }
                    Err(error) => tracing::warn!("Couldn't dial peer ({}): {:?}", peer_addr, error),
                };
            } else {
                tracing::warn!("Couldn't dial peer with address: {}", peer_addr);
            }
        });

        Ok(())
    }

    pub async fn loop_stream(&mut self) -> Result<()> {
        // TODO: Remove stdin feature
        let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
        let topic = IdentTopic::new("testnet");

        loop {
            select! {
                // TODO: Remove stdin feature
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
                        for (peer_id, _multiaddr) in list {
                            tracing::debug!("mDNS discovered a new peer: {}", peer_id);
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            tracing::debug!("mDNS discover peer has expired: {}", peer_id);
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => tracing::info!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),

                    _ => {}
                }
            }
        }
    }
}
