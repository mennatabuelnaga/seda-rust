use near_sdk::{Balance, MockedBlockchain, PromiseResult, RuntimeFeesConfig, VMConfig, VMContext};

pub fn staking() -> String {
    "staking".to_string()
}
pub fn alice() -> String {
    "alice".to_string()
}
pub fn bob() -> String {
    "bob".to_string()
}
pub fn owner() -> String {
    "owner".to_string()
}
pub fn token() -> String {
    "token".to_string()
}

pub fn ntoy(near_amount: Balance) -> Balance {
    near_amount * 10u128.pow(24)
}

/// Rounds to nearest
pub fn yton(yocto_amount: Balance) -> Balance {
    (yocto_amount + (5 * 10u128.pow(23))) / 10u128.pow(24)
}

#[macro_export]
macro_rules! assert_eq_in_near {
    ($a:expr, $b:expr) => {
        assert_eq!(yton($a), yton($b))
    };
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!(yton($a), yton($b), $c)
    };
}

pub fn testing_env_with_promise_results(context: VMContext, promise_result: PromiseResult) {
    near_sdk::env::set_blockchain_interface(MockedBlockchain::new(
        context,
        VMConfig::test(),
        RuntimeFeesConfig::test(),
        vec![promise_result],
        near_sdk::mock::with_mocked_blockchain(|b| b.take_storage()),
        Default::default(),
        None,
    ));
}
