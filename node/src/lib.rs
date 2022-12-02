mod app;
mod errors;
pub use errors::*;
mod event_queue;
mod event_queue_handler;
mod rpc;
mod runtime_job;

use actix::prelude::*;
use app::App;
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
        // TODO: add number of workers as config with default value
        let app = App::new(2).await.start();

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
