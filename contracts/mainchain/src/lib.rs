pub mod account;
pub mod block;
pub mod data_request;
pub mod data_request_test;
pub mod epoch;
pub mod fungible_token;
pub mod merkle;
pub mod node_registry;
pub mod node_registry_test;
pub mod staking;
pub mod storage;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, Vector},
    near_bindgen,
    AccountId,
    Balance,
    BorshStorageKey,
};

use crate::{
    account::Account,
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
    Accounts,
}

/// Contract global state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    seda_token:               AccountId,
    num_nodes:                u64,
    nodes:                    LookupMap<u64, Node>,
    data_request_accumulator: Vector<String>,
    num_blocks:               BlockHeight,
    block_ids_by_height:      LookupMap<BlockHeight, BlockId>,
    blocks_by_id:             LookupMap<BlockId, Block>,
    epoch:                    u64,
    accounts:                 UnorderedMap<AccountId, Account>,
    last_total_balance:       Balance,
}

impl Default for MainchainContract {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new(seda_token: AccountId) -> Self {
        let account_balance = 0; // TODO: fetch ft_balance_of this contract on initialization

        Self {
            seda_token,
            num_nodes: 0,
            nodes: LookupMap::new(MainchainStorageKeys::NumNodes),
            data_request_accumulator: Vector::<String>::new(MainchainStorageKeys::DataRequestAccumulator),
            num_blocks: 0,
            block_ids_by_height: LookupMap::new(MainchainStorageKeys::BlockIdsByHeight),
            blocks_by_id: LookupMap::new(MainchainStorageKeys::BlocksById),
            epoch: 0,
            accounts: UnorderedMap::new(MainchainStorageKeys::Accounts),
            last_total_balance: account_balance,
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
