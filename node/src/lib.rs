mod app;
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod p2p;
mod rpc;
mod runtime_job;

use actix::prelude::*;
use app::App;
use p2p::{P2PConfig, P2PServer};
use seda_chain_adapters::MainChainAdapterTrait;
use tracing::info;

use crate::app::Shutdown;

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
}

pub fn run<T: MainChainAdapterTrait>() {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        let app = App::new().await.start();

        // TODO: Use config for P2P Server
        let p2p_config = P2PConfig::default();

        // P2P initialization
        // TODO: most probably this process should be moved somewhere else
        actix::spawn(async move {
            let mut p2p_server = P2PServer::start_from_config(p2p_config)
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
