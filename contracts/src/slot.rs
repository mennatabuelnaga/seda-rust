use near_sdk::{env, near_bindgen};

use crate::{MainchainContract, MainchainContractExt};

pub const SECONDS_PER_SLOT: u64 = 12;

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_current_slot(&self) -> u64 {
        env::block_timestamp() / SECONDS_PER_SLOT
    }
}
