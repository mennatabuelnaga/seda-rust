mod app;
mod config;
mod host;
pub use config::*;
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod rpc;
mod runtime_adapter;
mod runtime_job;

use actix::prelude::*;
use app::App;
use seda_adapters::MainChainAdapterTrait;
use tracing::{error, info};

use crate::app::Shutdown;

#[cfg(test)]
#[path = ""]
pub mod test {
    mod event_queue_test;
}

pub fn run<T: MainChainAdapterTrait>(node_config: &NodeConfig, _main_chain_config: &T::Config) {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        // TODO: add number of workers as config with default value
        let app = App::new(node_config.runtime_worker_threads as usize).await.start();

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
