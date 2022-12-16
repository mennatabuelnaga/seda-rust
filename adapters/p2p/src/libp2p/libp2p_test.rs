use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use seda_config::NodeConfig;

use super::P2PServer;

#[tokio::test]
async fn p2p_service_works() {
    let config = NodeConfig::default();
    let mut p2p_service = P2PServer::start_from_config(&config.p2p_server_address, &config.p2p_known_peers)
        .await
        .expect("P2P swarm cannot be started");

    loop {
        match p2p_service.swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { .. } => {
                // listener address registered, we are good to go
                break;
            }
            _ => {
                tracing::error!("Unexpected event");
                panic!("Unexpected event")
            }
        }
    }
}
