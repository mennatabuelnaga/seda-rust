use std::sync::Arc;

use actix::prelude::*;
use parking_lot::RwLock;

use crate::{
    event_queue::{EventId, EventQueue},
    job_manager::StartJobManager,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Shutdown;

// Node Actor definition
pub struct App {
    pub event_queue:       Arc<RwLock<EventQueue>>,
    pub running_event_ids: Arc<RwLock<Vec<EventId>>>,
}

impl Actor for App {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Node starting...");
        let banner = r#"
         _____ __________  ___         ____  __  _____________
        / ___// ____/ __ \/   |       / __ \/ / / / ___/_  __/
        \__ \/ __/ / / / / /| |______/ /_/ / / / /\__ \ / /
       ___/ / /___/ /_/ / ___ /_____/ _, _/ /_/ /___/ // /
      /____/_____/_____/_/  |_|    /_/ |_|\____//____//_/
        "#;
        println!("{}", banner);

        ctx.notify(StartJobManager);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Node stopped");
    }
}

// Simple message handler for Ping message
impl Handler<Shutdown> for App {
    type Result = ();

    fn handle(&mut self, _msg: Shutdown, _ctx: &mut Context<Self>) {
        // Node stopping logic (for gracefull shutdown)

        System::current().stop();
    }
}
