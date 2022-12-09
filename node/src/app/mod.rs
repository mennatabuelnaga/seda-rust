use std::sync::Arc;

use actix::prelude::*;
use parking_lot::RwLock;
use seda_config::CONFIG;
use seda_runtime_adapters::HostAdapter;
use tracing::info;

use crate::{
    event_queue::{EventId, EventQueue},
    rpc::JsonRpcServer,
    runtime_job::RuntimeWorker,
};

mod job_manager;
mod shutdown;
pub use shutdown::Shutdown;
// Node Actor definition
pub struct App<HA: HostAdapter> {
    pub event_queue:       Arc<RwLock<EventQueue>>,
    pub running_event_ids: Arc<RwLock<Vec<EventId>>>,
    pub runtime_worker:    Addr<RuntimeWorker<HA>>,
    pub rpc_server:        JsonRpcServer,
}

impl<HA: HostAdapter> App<HA> {
    pub async fn new() -> Self {
        let config = CONFIG.read().await;
        // Okay to unwrap since CLI already checks if node section exists.
        let worker_threads = config.node.as_ref().unwrap().runtime_worker_threads.unwrap_or(2);
        let runtime_worker = SyncArbiter::start(worker_threads, move || RuntimeWorker { runtime: None });

        let rpc_server_address_default = "127.0.0.1:12345".to_string();
        let rpc_server_address = config
            .node
            .as_ref()
            .unwrap()
            .rpc_server_address
            .as_ref()
            .unwrap_or(&rpc_server_address_default);
        let rpc_server = JsonRpcServer::start(runtime_worker.clone(), rpc_server_address)
            .await
            .expect("Error starting jsonrpsee server");
        App {
            event_queue: Default::default(),
            running_event_ids: Default::default(),
            runtime_worker,
            rpc_server,
        }
    }
}

impl<HA: HostAdapter> Actor for App<HA> {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let banner = r#"
         _____ __________  ___         ____  __  _____________
        / ___// ____/ __ \/   |       / __ \/ / / / ___/_  __/
        \__ \/ __/ / / / / /| |______/ /_/ / / / /\__ \ / /
       ___/ / /___/ /_/ / ___ /_____/ _, _/ /_/ /___/ // /
      /____/_____/_____/_/  |_|    /_/ |_|\____//____//_/
        "#;
        info!("Node starting... \n{}", banner);

        info!("Starting Job Manager...");
        ctx.notify(job_manager::StartJobManager);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Node stopped");
    }
}
