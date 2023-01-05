use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct Node {
    pub owner:          String,
    pub pending_owner:  Option<String>,
    pub socket_address: String, // ip address and port
}
