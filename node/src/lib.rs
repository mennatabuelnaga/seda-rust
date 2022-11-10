mod app;
mod errors;
mod event_queue;
mod event_queue_handler;
mod job_manager;
mod p2p;
mod rpc;
mod runtime_job;
mod test_adapters;

use std::sync::Arc;

use actix::prelude::*;
use app::App;
use event_queue::EventQueue;
use parking_lot::RwLock;
use rpc::JsonRpcServer;

use crate::{app::Shutdown, p2p::p2p_listen, rpc::Stop};

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
    mod job_manager_test;
}

pub fn run(
    jsonrpc_server_address: Option<String>,
    p2p_server_address: Option<String>,
    known_peers: Option<Vec<String>>,
) {
    // TODO: add config (from CLI, config files and secrets from ENV)

    // Initialize actors inside system context
    let system = System::new();
    system.block_on(async {
        let app = App {
            event_queue:       Arc::new(RwLock::new(EventQueue::default())),
            running_event_ids: Arc::new(RwLock::new(Vec::new())),
        }
        .start();

        p2p_listen(peer_address).await.unwrap();
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
