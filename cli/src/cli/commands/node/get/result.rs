use serde::{Deserialize, Serialize};

// TODO move to sdk cause this can also be used in the WASM?
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct NodeResult {
    pub owner:          String,
    pub pending_owner:  Option<String>,
    pub socket_address: String, // ip address and port
}
