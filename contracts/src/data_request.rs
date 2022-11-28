use near_sdk::{env, log, near_bindgen, Promise};

use crate::{
    macros::manage_storage_deposit,
    merkle::{merklize, CryptoHash},
    MainchainContract,
    MainchainContractExt,
};

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn post_data_request(&mut self, data_request: String) {
        manage_storage_deposit!(self, self.data_request_accumulator.push(&data_request));
    }

    pub fn compute_merkle_root(&self) -> CryptoHash {
        let initial_gas = env::used_gas();

        let merkle_root = merklize(&self.data_request_accumulator.to_vec());

        log!("used gas: {}", u64::from(env::used_gas() - initial_gas));

        merkle_root
    }
}
