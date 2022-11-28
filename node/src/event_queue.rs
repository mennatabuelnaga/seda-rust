use serde::{Deserialize, Serialize};

pub type EventId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventData {
    // Tick types
    MainChainTick,
    CliCall(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id:   EventId,
    pub data: EventData,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EventQueue {
    items: Vec<Event>,
}

impl EventQueue {
    pub fn add(&mut self, event: Event) {
        self.items.push(event);
    }

    pub fn get_next(&mut self, skip_ids: &[String]) -> Option<Event> {
        for index in 0..self.items.len() {
            if skip_ids.contains(&self.items[index].id) {
                continue;
            }

            let item = self.items.remove(index);
            return Some(item);
        }

        None
    }
}
