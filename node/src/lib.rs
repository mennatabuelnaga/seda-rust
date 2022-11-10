mod app;
mod errors;
mod p2p;
mod rpc;
use actix::prelude::*;
use app::App;
use rpc::JsonRpcServer;

use crate::{app::Shutdown, p2p::p2p_listen, rpc::Stop};

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
