use std::sync::Arc;

use libp2p::{futures::StreamExt, swarm::SwarmEvent};
use parking_lot::RwLock;
use seda_config::P2PConfigInner;
use seda_runtime_sdk::p2p::{P2PCommand, P2PMessage};
use tokio::sync::mpsc::channel;

use super::P2PServer;
use crate::{libp2p::peer_list::PeerList, DiscoveryStatus};

#[tokio::test]
async fn p2p_service_works() {
    let (p2p_message_sender, _p2p_message_receiver) = channel::<P2PMessage>(100);
    let (_p2p_command_sender, p2p_command_receiver) = channel::<P2PCommand>(100);

    let p2p_config = P2PConfigInner::test_config();
    let discovery_status = Arc::new(RwLock::new(DiscoveryStatus::new(
        p2p_config.clone(),
        PeerList::from_vec(&p2p_config.p2p_known_peers),
    )));
    let mut p2p_service = P2PServer::new(
        discovery_status,
        p2p_config.clone(),
        p2p_message_sender,
        p2p_command_receiver,
    )
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
