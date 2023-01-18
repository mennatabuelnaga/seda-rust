use serde::{Deserialize, Serialize};

pub type EventId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventData {
    // Tick types
    ChainTick,
    CliCall(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id:   EventId,
    pub data: EventData,
}
