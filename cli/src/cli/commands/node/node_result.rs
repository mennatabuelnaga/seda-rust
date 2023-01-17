use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct NodeResult {
    pub owner:          String,
    pub pending_owner:  Option<String>,
    pub socket_address: String, // ip address and port
}
