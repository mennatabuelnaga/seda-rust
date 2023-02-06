use near_sdk::{env, near_bindgen};

use crate::{MainchainContract, MainchainContractExt};

pub const NEAR_BLOCKS_PER_SEDA_SLOT: u64 = 10; // at 1.2s/block, 12s/slot

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_current_slot(&self) -> u64 {
        env::block_height() / NEAR_BLOCKS_PER_SEDA_SLOT
    }
}
