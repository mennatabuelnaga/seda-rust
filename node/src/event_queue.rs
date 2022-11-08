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
