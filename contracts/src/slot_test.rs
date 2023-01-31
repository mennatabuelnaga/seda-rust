#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::MainchainContract;

    fn get_context(block_timestamp: u64) -> VMContext {
        VMContextBuilder::new()
            .block_timestamp(block_timestamp)
            .is_view(true)
            .build()
    }

    #[test]
    fn get_current_slot() {
        let contract = MainchainContract::new();

        testing_env!(get_context(0)); // unix epoch
        assert_eq!(contract.get_current_slot(), 0);

        testing_env!(get_context(11)); // unix epoch + 11 seconds (same slot)
        assert_eq!(contract.get_current_slot(), 0);

        testing_env!(get_context(12)); //  // unix epoch + 12 seconds (new slot)
        assert_eq!(contract.get_current_slot(), 1);
    }

    #[test]
    fn get_current_epoch() {
        let contract = MainchainContract::new();

        testing_env!(get_context(0)); // unix epoch
        assert_eq!(contract.get_current_epoch(), 0);

        testing_env!(get_context(12)); // unix epoch + 12 seconds (new slot but same epoch)
        assert_eq!(contract.get_current_epoch(), 0);
        assert_eq!(contract.get_current_slot(), 1);

        testing_env!(get_context(12 * 32)); // unix epoch + 12 seconds (new epoch)
        assert_eq!(contract.get_current_epoch(), 1);
        assert_eq!(contract.get_current_slot(), 32);
    }
}
