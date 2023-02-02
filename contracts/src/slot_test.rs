#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::{epoch::SLOTS_PER_EPOCH, slot::NEAR_BLOCKS_PER_SEDA_SLOT, MainchainContract};

    fn get_context(block_index: u64) -> VMContext {
        VMContextBuilder::new().block_index(block_index).is_view(true).build()
    }

    #[test]
    fn get_current_slot() {
        let contract = MainchainContract::new();

        testing_env!(get_context(0)); // block 0
        assert_eq!(contract.get_current_slot(), 0); // slot 0

        testing_env!(get_context(NEAR_BLOCKS_PER_SEDA_SLOT - 1)); // block 9
        assert_eq!(contract.get_current_slot(), 0); // slot 0

        testing_env!(get_context(NEAR_BLOCKS_PER_SEDA_SLOT)); // block 10
        assert_eq!(contract.get_current_slot(), 1); // slot 1
    }

    #[test]
    fn get_current_epoch() {
        let contract = MainchainContract::new();

        testing_env!(get_context(0)); // block 0
        assert_eq!(contract.get_current_epoch(), 0); // epoch 0

        testing_env!(get_context(NEAR_BLOCKS_PER_SEDA_SLOT * SLOTS_PER_EPOCH - 1)); // block 319
        assert_eq!(contract.get_current_slot(), 31); // slot 31
        assert_eq!(contract.get_current_epoch(), 0); // epoch 0

        testing_env!(get_context(NEAR_BLOCKS_PER_SEDA_SLOT * SLOTS_PER_EPOCH)); // block 320
        assert_eq!(contract.get_current_slot(), 32); // slot 32
        assert_eq!(contract.get_current_epoch(), 1); // epoch 1
    }
}
