#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::{merkle::CryptoHash, MainchainContract};

    fn get_context(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .build()
    }
    fn get_context_with_deposit(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .attached_deposit(1_400_000_000_000_000_000_000) // required for post_data_request()
            .build()
    }

    #[test]
    fn create_block() {
        let mut contract = MainchainContract::new();

        // check current block
        testing_env!(get_context("bob_near".to_string()));
        assert_eq!(contract.get_latest_block_id(), CryptoHash::default());

        // post data request
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.post_data_request("data_request_1".to_string());
        contract.post_data_request("data_request_2".to_string());
        contract.post_data_request("data_request_3".to_string());

        // create block
        contract.create_block();

        // check current block
        testing_env!(get_context("bob_near".to_string()));
        assert_ne!(contract.get_latest_block_id(), CryptoHash::default());
    }

    #[test]
    fn create_multiple_blocks() {
        let mut contract = MainchainContract::new();

        // post data request for first block
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.post_data_request("data_request_1".to_string());
        contract.post_data_request("data_request_2".to_string());
        contract.post_data_request("data_request_3".to_string());

        // create and check first block
        contract.create_block();
        testing_env!(get_context("bob_near".to_string()));
        let block_id_1 = contract.get_latest_block_id();
        assert_ne!(block_id_1, CryptoHash::default());

        // post data request for second block
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.post_data_request("data_request_4".to_string());
        contract.post_data_request("data_request_5".to_string());
        contract.post_data_request("data_request_6".to_string());

        // create and check second block
        contract.create_block();
        testing_env!(get_context("bob_near".to_string()));
        let block_id_2 = contract.get_latest_block_id();
        assert_ne!(block_id_2, block_id_1);
    }
}
