use std::{error::Error, ops::Mul};

use libp2p::{
    futures::StreamExt,
    identity,
    ping,
    swarm::{dial_opts::DialOpts, Swarm, SwarmEvent},
    Multiaddr,
    NetworkBehaviour,
    PeerId,
};

pub async fn p2p_listen(peer_address: Option<String>) -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("{:?}", std::env::args().nth(2));

    if let Some(addr) = peer_address {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}
