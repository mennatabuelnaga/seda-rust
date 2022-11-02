use actix::{Handler, Message};

use crate::{app::App, event_queue::Event};

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddEventToQueue {
    pub event: Event,
}

impl Handler<AddEventToQueue> for App {
    type Result = ();

    fn handle(&mut self, msg: AddEventToQueue, _ctx: &mut Self::Context) -> Self::Result {
        let mut event_queue = self.event_queue.write();

        event_queue.add(msg.event);
    }
}
