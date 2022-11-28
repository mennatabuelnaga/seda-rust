use near_sdk::{env, log, near_bindgen};

use crate::{
    merkle::{merklize, CryptoHash},
    MainchainContract,
    MainchainContractExt,
};

macro_rules! require_storage_deposit {
    ($self:ident, $expression:expr) => {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();
        
        // execute the expression
        $expression;

        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
    };
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    pub fn post_data_request(&mut self, data_request: String) {
        require_storage_deposit!(self, {
            self.data_request_accumulator.push(&data_request);
        });
    }

    pub fn compute_merkle_root(&self) -> CryptoHash {
        let initial_gas = env::used_gas();

        let merkle_root = merklize(&self.data_request_accumulator.to_vec());

        log!("used gas: {}", u64::from(env::used_gas() - initial_gas));

        merkle_root
    }
}
