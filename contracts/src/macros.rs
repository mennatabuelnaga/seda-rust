macro_rules! require_storage_deposit {
    ($self:ident, $initial_storage_usage:ident) => {
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - $initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
    };
}

macro_rules! refund_storage_deposit {
    ($self:ident, $initial_storage_usage:ident) => {
        let storage_cost = env::storage_byte_cost() * u128::from($initial_storage_usage - env::storage_usage());
        Promise::new(env::signer_account_id()).transfer(storage_cost);
        log!(
            "Refunding {} for storage deposit to {}",
            storage_cost,
            env::signer_account_id()
        );
    };
}

macro_rules! manage_storage_deposit {
    // storage is assumed to increase
    ($self:ident,true, $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        require_storage_deposit!($self, initial_storage_usage);
    };

    // storage is assumed to decrease
    ($self:ident,false, $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        refund_storage_deposit!($self, initial_storage_usage);
    };

    // storage could increase or decrease
    ($self:ident, $expression:expr) => {
        let initial_storage_usage = env::storage_usage();
        $expression;
        if (env::storage_usage() > initial_storage_usage) {
            require_storage_deposit!($self, initial_storage_usage);
        } else {
            refund_storage_deposit!($self, initial_storage_usage);
        }
    };
}

pub(crate) use manage_storage_deposit;
pub(crate) use refund_storage_deposit;
pub(crate) use require_storage_deposit;
