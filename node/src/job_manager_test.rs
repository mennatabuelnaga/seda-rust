use std::sync::Arc;

use actix::{Actor, System};
use parking_lot::RwLock;

use crate::{
    app::App,
    event_queue::{Event, EventData, EventQueue},
    event_queue_handler::AddEventToQueue,
    job_manager::StartJobManager,
};

#[test]
fn test_job_manager() {
    let system = System::new();

    system.block_on(async {
        let app = App {
            event_queue:       Arc::new(RwLock::new(EventQueue::default())),
            running_event_ids: Arc::new(RwLock::new(Vec::new())),
        }
        .start();

        app.send(AddEventToQueue {
            event: Event {
                id:   "test".to_string(),
                data: EventData::MainChainTick,
            },
        })
        .await;
        app.send(StartJobManager).await;
    });

    system.run();
}
