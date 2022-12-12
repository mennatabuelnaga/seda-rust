use near_sdk::{env, near_bindgen};

use crate::{manage_storage_deposit, MainchainContract, MainchainContractExt};

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn get_epoch(&self) -> u64 {
        self.epoch
    }

    // TODO: make permissioned
    pub fn increment_epoch(&mut self) {
        manage_storage_deposit!(self, "require", {
            self.epoch += 1;
        });
    }
}
