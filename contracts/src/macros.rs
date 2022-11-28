macro_rules! manage_storage_deposit {
    ($self:ident, $expression:expr) => {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

        // execute the expression
        $expression;

        if (env::storage_usage() > initial_storage_usage) {
            // require storage deposit
            let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
            assert!(
                storage_cost <= env::attached_deposit(),
                "Insufficient storage, need {}",
                storage_cost
            );
        } else {
            // refund storage deposit
            let storage_cost = env::storage_byte_cost() * u128::from(initial_storage_usage - env::storage_usage());
            Promise::new(env::signer_account_id()).transfer(storage_cost);
            log!(
                "Refunding {} for storage deposit to {}",
                storage_cost,
                env::signer_account_id()
            );
        }
    };
}

pub(crate) use manage_storage_deposit;
