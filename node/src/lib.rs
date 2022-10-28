mod app;
mod config;

use actix::prelude::*;
use app::App;

use crate::app::Shutdown;

pub fn run() {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        let app = App.start();

        // Intercept ctrl+c to stop gracefully the system
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("failed to listen for event");
            println!("\nStopping the node gracefully...");

            app.do_send(Shutdown);
        });
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));
}
