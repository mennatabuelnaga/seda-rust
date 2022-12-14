use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use tracing::instrument;

use super::{P2PConfig, P2PServer};

#[tokio::test]
#[instrument]
async fn p2p_service_works() {
    let p2p_config = P2PConfig::default();
    let mut p2p_service = P2PServer::start_from_config(p2p_config)
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
