use libp2p::{
    futures::StreamExt,
    identity::{self},
    ping::{self, Behaviour},
    swarm::{Swarm, SwarmEvent},
    Multiaddr,
    PeerId,
};

use super::errors::Result;

pub struct P2PConfig {
    pub server_address: Option<String>,
    pub known_peers:    Vec<String>,
}

pub struct P2PServer {
    pub config:    P2PConfig,
    pub local_key: identity::Keypair,
    pub swarm:     Swarm<Behaviour>,
}

impl P2PServer {
    pub async fn start_from_config(config: P2PConfig) -> Result<Self> {
        // Generate Peer ID
        // TODO: Support peer id from config and storage
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Build Swarm
        let transport = libp2p::development_transport(local_key.clone()).await?;
        let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
        let mut swarm = Swarm::new(transport, behaviour, PeerId::from(local_key.public()));
        swarm.listen_on(
            config
                .server_address
                .as_ref()
                .unwrap_or(&"/ip4/0.0.0.0/tcp/0".to_string())
                .parse()?,
        )?;

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
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
                SwarmEvent::Behaviour(event) => println!("{:?}", event),
                _ => {}
            }
        }
    }
}
