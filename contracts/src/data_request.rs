use near_primitives::merkle::merklize;
use near_sdk::{env, log, near_bindgen};

use crate::{MainchainContract, MainchainContractExt};

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn post_data_request(&mut self, data_request: String) {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

        self.data_request_accumulator.insert(&data_request);

        // check for storage deposit
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
    }

    pub fn compute_merkle_root(&self) -> String {
        let initial_gas = env::used_gas();

        // TODO: sort data requests
        let data_requests: Vec<String> = self.data_request_accumulator.iter().collect();
        let merkle_root = merklize(&data_requests);

        log!("used gas: {}", u64::from(env::used_gas() - initial_gas));

        merkle_root.0.to_string()
    }
}
