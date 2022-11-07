pub type EventId = String;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum EventData {
    // Tick types
    MainChainTick,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub id:   EventId,
    pub data: EventData,
}

#[derive(Default, Debug)]
pub struct EventQueue {
    items: Vec<Event>,
}

impl EventQueue {
    pub fn add(&mut self, event: Event) {
        self.items.push(event);
    }

    pub fn get_next(&mut self, skip_ids: Vec<EventId>) -> Option<Event> {
        for (index, item) in self.items.clone().iter().enumerate() {
            if skip_ids.contains(&item.id) {
                continue;
            }

            self.items.remove(index);
            return Some(item.clone());
        }

        None
    }
}
