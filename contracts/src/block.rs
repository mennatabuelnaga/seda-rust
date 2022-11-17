use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    near_bindgen,
};

use crate::{merkle::CryptoHash, MainchainContract, MainchainContractExt};

pub type BlockHeight = u8;
pub type BlockId = CryptoHash;

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct BlockHeader {
    pub height:     BlockHeight,
    pub state_root: CryptoHash,
}

/// Block data without merkle roots (stored on contract)
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Block {
    pub header:       BlockHeader,
    pub transactions: Vec<String>,
}

/// Block data using merkle roots (used to calculate block id)
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MerklizedBlock {
    pub prev_root:    BlockId,
    pub header:       BlockHeader,
    pub transactions: CryptoHash,
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_latest_block(&self) -> BlockId {
        self.blocks.get(&self.num_blocks).unwrap_or_default()
    }

    pub fn create_block(&mut self) {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

        self.num_blocks += 1;

        let header = BlockHeader {
            height:     self.num_blocks,
            state_root: CryptoHash::default(), // TODO
        };

        // create block
        let block = Block {
            header:       header.clone(),
            transactions: self.data_request_accumulator.to_vec(),
        };

        // calculate block id
        let block_id = CryptoHash::hash_borsh(&MerklizedBlock {
            prev_root: self.get_latest_block(),
            header,
            transactions: self.compute_merkle_root(),
        });

        // store block
        self.blocks_by_id.insert(&block_id, &block);
        self.blocks.insert(&self.num_blocks, &block_id);

        // clear data request accumulator
        self.data_request_accumulator.clear();

        // check for storage deposit
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
    }
}
