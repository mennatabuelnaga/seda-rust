use std::sync::Arc;

use actix::prelude::*;
use parking_lot::RwLock;
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
pub struct App {
    pub event_queue:       Arc<RwLock<EventQueue>>,
    pub running_event_ids: Arc<RwLock<Vec<EventId>>>,
    pub runtime_worker:    Addr<RuntimeWorker>,
    pub rpc_server:        JsonRpcServer,
}

impl App {
    pub async fn new(worker_threads: usize) -> Self {
        let runtime_worker = SyncArbiter::start(worker_threads, move || RuntimeWorker { runtime: None });
        let rpc_server = JsonRpcServer::start(runtime_worker.clone())
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

impl Actor for App {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Node starting...");
        let banner = r#"
         _____ __________  ___         ____  __  _____________
        / ___// ____/ __ \/   |       / __ \/ / / / ___/_  __/
        \__ \/ __/ / / / / /| |______/ /_/ / / / /\__ \ / /
       ___/ / /___/ /_/ / ___ /_____/ _, _/ /_/ /___/ // /
      /____/_____/_____/_/  |_|    /_/ |_|\____//____//_/
        "#;
        info!("{}", banner);

        info!("Starting Job Manager...");
        ctx.notify(job_manager::StartJobManager);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Node stopped");
    }
}
