use actix::{Handler, Message};
use serde::{Deserialize, Serialize};
use seda_chain_adapters::MainChainAdapterTrait;

use crate::{ event_queue::Event, app::App};

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct AddEventToQueue {
    pub event: Event,
}

impl<MC> Handler<AddEventToQueue> for App<MC>
where MC: MainChainAdapterTrait {
    type Result = ();

    fn handle(&mut self, msg: AddEventToQueue, _ctx: &mut Self::Context) -> Self::Result {
        let mut event_queue = self.event_queue.write();

        event_queue.add(msg.event);
    }
}
