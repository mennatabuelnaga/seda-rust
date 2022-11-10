mod app;
mod errors;
mod p2p;
mod rpc;
use actix::prelude::*;
use app::App;
use p2p::{P2PConfig, P2PServer};
use rpc::JsonRpcServer;

use crate::{app::Shutdown, rpc::Stop};

pub fn run(
    jsonrpc_server_address: Option<String>,
    p2p_server_address: Option<String>,
    known_peers: Option<Vec<String>>,
) {
    // TODO: add config (from CLI, config files and secrets from ENV)

    // Initialize actors inside system context
    let system = System::new();
    system.block_on(async {
        // Node application
        let app = App.start();

        // Json-RPC Server
        let rpc_server = JsonRpcServer::build(&jsonrpc_server_address.unwrap_or_else(|| "127.0.0.1:12345".to_string()))
            .await
            .expect("Error starting jsonrpsee server")
            .start();

        // P2P Server
        let p2p_config = P2PConfig {
            server_address: p2p_server_address,
            known_peers:    known_peers.unwrap_or_default(),
        };

        // P2P initialization
        // TODO: most probably this process should be moved somewhere else
        tokio::spawn(async move {
            let mut p2p_server = P2PServer::start_from_config(p2p_config)
                .await
                .expect("P2P swarm cannot be started");
            p2p_server.dial_peers().await.expect("P2P dial behaviour failed");
            p2p_server.loop_stream().await.expect("P2P listen failed");
        });

        // Intercept ctrl+c to stop gracefully the system
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            println!("\nStopping the node gracefully...");

            if let Err(error) = rpc_server.send(Stop).await {
                println!("Error while stopping RPC server ({}).", error);
            }

            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
