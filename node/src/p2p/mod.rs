mod behaviour;
mod transport;

use async_std::io;
use libp2p::{
    futures::{prelude::*, select, StreamExt},
    gossipsub::{GossipsubEvent, IdentTopic},
    identity::{self},
    mdns::MdnsEvent,
    swarm::{Swarm, SwarmEvent},
    Multiaddr,
    PeerId,
};

use self::behaviour::SedaBehaviour;
use super::errors::Result;
use crate::p2p::{behaviour::SedaBehaviourEvent, transport::build_tcp_transport};

pub struct P2PConfig {
    pub server_address: String,
    pub known_peers:    Vec<String>,
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
        println!("Local peer id: {:?}", local_peer_id);

        // Create a random PeerId
        let transport_key_pair = identity::Keypair::generate_ed25519();
        let transport_peer_id = PeerId::from(transport_key_pair.public());
        println!("Random peer id: {transport_peer_id:?}");

        let transport = build_tcp_transport(transport_key_pair);
        let seda_behaviour = SedaBehaviour::new(&config, &local_key).await;

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
                        println!("Dialed {}", peer_addr);
                    }
                    Err(error) => println!("Couldn't dial peer ({}): {:?}", peer_addr, error),
                };
            } else {
                println!("Couldn't dial peer with address: {}", peer_addr);
            }
        });

        Ok(())
    }

    pub async fn loop_stream(&mut self) -> Result<()> {
        let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
        let topic = IdentTopic::new("test-net");

        loop {
            select! {
                line = stdin.select_next_some() => {
                    if let Err(e) = self.swarm
                        .behaviour_mut().gossipsub
                        .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
                        println!("Publish error: {e:?}");
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {}", peer_id);
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {}", peer_id);
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(SedaBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => println!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),

                    _ => {}
                }
            }
        }
    }
}
