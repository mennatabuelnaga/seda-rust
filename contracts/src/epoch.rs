use near_sdk::{env, near_bindgen};

use crate::{slot::SECONDS_PER_SLOT, MainchainContract, MainchainContractExt};

const SLOTS_PER_EPOCH: u64 = 32;

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_current_epoch(&self) -> u64 {
        env::block_timestamp() / (SECONDS_PER_SLOT * SLOTS_PER_EPOCH)
    }
}
