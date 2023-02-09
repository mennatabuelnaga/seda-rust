#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use crate::{dao::UpdateConfig, MainchainContract};

    fn get_context(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .build()
    }
    fn new_contract() -> MainchainContract {
        MainchainContract::new(
            "dao_near".to_string().try_into().unwrap(),
            "seda_token".to_string().try_into().unwrap(),
        )
    }

    #[test]
    fn update_config() {
        let mut contract = new_contract();
        testing_env!(get_context("dao_near".to_string()));
        contract.update_config(UpdateConfig::MinimumStake, 100);
    }

    #[test]
    #[should_panic(expected = "Only DAO can call this method")]
    fn update_config_wrong_account() {
        let mut contract = new_contract();
        testing_env!(get_context("bob_near".to_string()));
        contract.update_config(UpdateConfig::MinimumStake, 100);
    }
}
