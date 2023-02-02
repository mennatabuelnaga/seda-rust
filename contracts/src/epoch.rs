use near_sdk::{env, near_bindgen};

use crate::{slot::NEAR_BLOCKS_PER_SEDA_SLOT, MainchainContract, MainchainContractExt};

pub const SLOTS_PER_EPOCH: u64 = 32;

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_current_epoch(&self) -> u64 {
        env::block_height() / (NEAR_BLOCKS_PER_SEDA_SLOT * SLOTS_PER_EPOCH)
    }
}
