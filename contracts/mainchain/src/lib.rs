pub mod block;
pub mod data_request;
pub mod merkle;
pub mod node_registry;
pub mod storage;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, Vector},
    near_bindgen,
    BorshStorageKey,
};

use crate::{
    block::{Block, BlockHeight, BlockId},
    node_registry::Node,
};

/// Collection keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
    DataRequestAccumulator,
    BlockIdsByHeight,
    BlocksById,
}

/// Contract global state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    num_nodes:                u64,
    nodes:                    LookupMap<u64, Node>,
    data_request_accumulator: Vector<String>,
    num_blocks:               BlockHeight,
    block_ids_by_height:      LookupMap<BlockHeight, BlockId>,
    blocks_by_id:             LookupMap<BlockId, Block>,
}

impl Default for MainchainContract {
    fn default() -> Self {
        Self::new()
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new() -> Self {
        Self {
            num_nodes:                0,
            nodes:                    LookupMap::new(MainchainStorageKeys::NumNodes),
            data_request_accumulator: Vector::<String>::new(MainchainStorageKeys::DataRequestAccumulator),
            num_blocks:               0,
            block_ids_by_height:      LookupMap::new(MainchainStorageKeys::BlockIdsByHeight),
            blocks_by_id:             LookupMap::new(MainchainStorageKeys::BlocksById),
        }
    }
}

#[cfg(test)]
#[path = ""]
mod tests {
    mod block_test;
    mod data_request_test;
    mod node_registry_test;
}
