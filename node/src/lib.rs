mod app;
mod config;
pub use config::*;
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod job_manager;
mod rpc;
mod runtime_job;
mod test_adapters;

use std::sync::Arc;

use actix::prelude::*;
use app::App;
use event_queue::EventQueue;
use job_manager::StartJobManager;
use parking_lot::RwLock;
use rpc::JsonRpcServer;
use runtime_job::RuntimeWorker;
use seda_adapters::MainChainAdapterTrait;
use tracing::{error, info};

use crate::{app::Shutdown, rpc::Stop};

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
    mod job_manager_test;
}

pub fn run<T: MainChainAdapterTrait>(node_config: &NodeConfig, main_chain_config: &T::Config) {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        let app = App {
            event_queue:       Arc::new(RwLock::new(EventQueue::default())),
            running_event_ids: Arc::new(RwLock::new(Vec::new())),
        }
        .start();

        // TODO: use config param for setting the number of threads
        let runtime_worker = SyncArbiter::start(2, move || RuntimeWorker { runtime: None });
        app.do_send(StartJobManager {
            runtime_worker: runtime_worker.clone(),
        });

        let rpc_server =
            JsonRpcServer::build::<T>(main_chain_config, node_config.server_address.as_ref().expect("todo"))
                .await
                .expect("Error starting jsonrpsee server")
                .start();

        // Intercept ctrl+c to stop gracefully the system
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            info!("\nStopping the node gracefully...");

            if let Err(error) = rpc_server.send(Stop).await {
                error!("Error while stopping RPC server ({}).", error);
            }

            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
