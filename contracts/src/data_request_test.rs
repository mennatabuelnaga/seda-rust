#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::MainchainContract;

    fn get_context_with_deposit(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .attached_deposit(1_360_000_000_000_000_000_000) // required for post_data_request()
            .build()
    }

    #[test]
    fn post_data_request() {
        let mut contract = MainchainContract::new();

        // post data request
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.post_data_request("data_request_1".to_string());
        contract.post_data_request("data_request_2".to_string());
    }
}
