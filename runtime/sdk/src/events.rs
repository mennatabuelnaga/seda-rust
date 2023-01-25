use serde::{Deserialize, Serialize};

use crate::p2p::P2PMessage;

pub type EventId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventData {
    // Tick types
    ChainTick,
    P2PMessage(P2PMessage),
    CliCall(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id:   EventId,
    pub data: EventData,
}
