mod app;
mod rpc;
use actix::prelude::*;
use app::App;
use rpc::JsonRpcServer;

use crate::{app::Shutdown, rpc::Stop};

pub fn run() {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        let app = App.start();
        let rpc_server = JsonRpcServer::build()
            .await
            .expect("Error starting jsonrpsee server")
            .start();

        // Intercept ctrl+c to stop gracefully the system
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            println!("\nStopping the node gracefully...");
            rpc_server.do_send(Stop);
            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
