use near_sdk::{env, log, Promise};

use crate::MainchainContract;

#[macro_export]
macro_rules! manage_storage_deposit {
    // storage is assumed to increase
    ($self:ident,"require", $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        $self.require_storage_deposit(initial_storage_usage);
    };

    // storage is assumed to decrease
    ($self:ident,"refund", $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        $self.refund_storage_deposit(initial_storage_usage);
    };

    // storage could increase or decrease
    ($self:ident, $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        if (env::storage_usage() > initial_storage_usage) {
            $self.require_storage_deposit(initial_storage_usage);
        } else {
            $self.refund_storage_deposit(initial_storage_usage);
        }
    };
}

/// Contract private methods
impl MainchainContract {
    pub fn require_storage_deposit(&self, initial_storage_usage: u64) {
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
    }

    pub fn refund_storage_deposit(&self, initial_storage_usage: u64) {
        let storage_cost = env::storage_byte_cost() * u128::from(initial_storage_usage - env::storage_usage());
        Promise::new(env::signer_account_id()).transfer(storage_cost);
        log!(
            "Refunding {} for storage deposit to {}",
            storage_cost,
            env::signer_account_id()
        );
    }
}
