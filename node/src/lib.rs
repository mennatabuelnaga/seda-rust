mod app;

use std::sync::Arc;

use app::{p2p_message_handler::P2PMessageHandler, App};
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod rpc;
mod runtime_job;

mod host;
use actix::prelude::*;
pub(crate) use host::*;
pub use host::{ChainCall, ChainView};
use parking_lot::RwLock;
use seda_config::{ChainConfigs, NodeConfig};
use seda_p2p::{libp2p::P2PServer, PeerList};
use seda_runtime_sdk::p2p::{P2PCommand, P2PMessage};
use tokio::sync::mpsc::channel;
use tracing::info;

use crate::app::Shutdown;

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
}

pub fn run(seda_server_address: &str, config: NodeConfig, chain_configs: ChainConfigs) {
    let system = System::new();
    // Initialize actors inside system context
    system.block_on(async {
        let (p2p_message_sender, p2p_message_receiver) = channel::<P2PMessage>(100);
        let (p2p_command_sender, p2p_command_receiver) = channel::<P2PCommand>(100);

        let known_peers = Arc::new(RwLock::new(PeerList::from_vec(&config.p2p_known_peers)));

        // TODO: add number of workers as config with default value
        let app = App::<RuntimeAdapter>::new(
            config.clone(),
            seda_server_address,
            chain_configs,
            p2p_command_sender,
            known_peers.clone(),
        )
        .await
        .start();

        // TODO: Use config for P2P Server
        let mut p2p_server = P2PServer::start_from_config(
            config.clone(),
            &config.p2p_server_address,
            known_peers,
            p2p_message_sender,
            p2p_command_receiver,
        )
        .await
        .expect("P2P swarm cannot be started");

        p2p_server.dial_peers().await.expect("P2P dial behaviour failed");

        // P2P initialization
        // TODO: most probably this process should be moved somewhere else
        actix::spawn(async move {
            p2p_server.loop_stream().await.expect("P2P Loop failed");
        });

        // Listens for p2p messages and sents the to the event queue
        let mut p2p_message_handler = P2PMessageHandler::new(p2p_message_receiver, app.clone());
        actix::spawn(async move {
            p2p_message_handler.listen().await;
        });

        // Intercept ctrl+c to stop gracefully the system
        actix::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            info!("\nStopping the node gracefully...");

            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
