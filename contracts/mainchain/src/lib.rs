pub mod block;
pub mod consts;
pub mod dao;
pub mod data_request;
pub mod epoch;
pub mod fungible_token;
pub mod merkle;
pub mod node_registry;
pub mod slot;
pub mod storage;
pub mod verify;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, Vector},
    env,
    near_bindgen,
    AccountId,
    Balance,
    BorshStorageKey,
    PanicOnDefault,
};

use crate::{
    block::{Block, BlockHeight, BlockId},
    node_registry::Node,
};

/// Collection keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    Nodes,
    DataRequestAccumulator,
    BlockIdsByHeight,
    BlocksById,
}

/// Contract global state
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    dao:                      AccountId,
    config:                   dao::Config,
    seda_token:               AccountId,
    nodes:                    UnorderedMap<AccountId, Node>,
    data_request_accumulator: Vector<String>,
    num_blocks:               BlockHeight,
    block_ids_by_height:      LookupMap<BlockHeight, BlockId>,
    blocks_by_id:             LookupMap<BlockId, Block>,
    last_total_balance:       Balance,
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new(dao: AccountId, seda_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(seda_token.as_bytes()),
            "The SEDA token account ID is invalid"
        );
        Self {
            dao,
            config: dao::Config::default(),
            seda_token,
            nodes: UnorderedMap::new(MainchainStorageKeys::Nodes),
            data_request_accumulator: Vector::<String>::new(MainchainStorageKeys::DataRequestAccumulator),
            num_blocks: 0,
            block_ids_by_height: LookupMap::new(MainchainStorageKeys::BlockIdsByHeight),
            blocks_by_id: LookupMap::new(MainchainStorageKeys::BlocksById),
            last_total_balance: 0,
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
#[path = ""]
mod tests {
    mod dao_test;
    mod data_request_test;
    mod node_registry_test;
    mod slot_test;
    mod verify_test;
}
