mod app;

use actix::prelude::*;
use app::App;
pub fn run() {
    let system = System::new();

    // Initialize actors inside system context
    system.block_on(async {
        let app = App.start();
    });

    system.run().expect("todo");
}
