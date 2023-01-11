mod app;
use app::App;
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod rpc;
mod runtime_job;

use actix::prelude::*;
use seda_config::{ChainConfigs, NodeConfig};
use seda_p2p_adapters::libp2p::P2PServer;
use seda_runtime_adapters::RuntimeAdapter;
use tracing::info;

use crate::app::Shutdown;

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
}

pub fn run(config: NodeConfig, chain_configs: ChainConfigs) {
    let system = System::new();
    // Initialize actors inside system context
    system.block_on(async {
        // TODO: add number of workers as config with default value
        let app = App::<RuntimeAdapter>::new(config.runtime_worker_threads, &config.rpc_server_address, chain_configs)
            .await
            .start();
        dbg!("made app");

        // TODO: Use config for P2P Server

        // P2P initialization
        // TODO: most probably this process should be moved somewhere else
        actix::spawn(async move {
            let mut p2p_server = P2PServer::start_from_config(&config.p2p_server_address, &config.p2p_known_peers)
                .await
                .expect("P2P swarm cannot be started");
            p2p_server.dial_peers().await.expect("P2P dial behaviour failed");
            p2p_server.loop_stream().await.expect("P2P listen failed");
        });

        // Intercept ctrl+c to stop gracefully the system
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            info!("\nStopping the node gracefully...");

            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
