#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};
    use crate::RewardFeeFraction;
    use crate::MainchainContract;

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
    fn new_contract() -> MainchainContract {
        MainchainContract::new("token_near".to_string().try_into().unwrap(), RewardFeeFraction {
            numerator: 10,
            denominator: 100,
        },)
    }

    #[test]
    fn post_data_request() {
        let mut contract = new_contract();

        // post data request
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.post_data_request("data_request_1".to_string());
        contract.post_data_request("data_request_2".to_string());
        contract.post_data_request("data_request_3".to_string());

        // compute merkle root
        testing_env!(get_context("bob_near".to_string()));
        contract.compute_merkle_root();
    }

    #[should_panic(expected = "Insufficient storage, need 670000000000000000000")]
    #[test]
    fn post_data_request_no_deposit() {
        let mut contract = new_contract();

        // post data request
        testing_env!(get_context("bob_near".to_string()));
        contract.post_data_request("data_request_1".to_string());
    }

    #[test]
    fn merkle_gas_tests() {
        let mut contract = new_contract();

        for i in 0..300 {
            testing_env!(get_context_with_deposit("bob_near".to_string()));
            contract.post_data_request(format!("data_request_{}", i));
            testing_env!(get_context("bob_near".to_string()));
            contract.compute_merkle_root();
        }
    }
}
