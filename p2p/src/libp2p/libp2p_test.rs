use futures::{channel::mpsc, StreamExt};
use libp2p::swarm::SwarmEvent;
use seda_config::NodeConfigInner;
use seda_runtime_sdk::p2p::{P2PCommand, P2PMessage};

use super::P2PServer;

#[tokio::test]
async fn p2p_service_works() {
    let (p2p_message_sender, _p2p_message_receiver) = mpsc::channel::<P2PMessage>(0);
    let (_p2p_command_sender, p2p_command_receiver) = mpsc::channel::<P2PCommand>(0);

    // TODO p2p should have its own config section.
    let config = NodeConfigInner::test_config();
    let mut p2p_service = P2PServer::start_from_config(
        &config.p2p_server_address,
        &config.p2p_known_peers,
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
