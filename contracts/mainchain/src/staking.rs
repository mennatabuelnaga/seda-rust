use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseOrValue};

use crate::{MainchainContract, MainchainContractExt};

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[private] // require caller to be this contract
    pub fn deposit(&mut self, amount: u128, account_id: AccountId) -> PromiseOrValue<U128> {
        let mut account = self.get_account(&account_id);
        account.unstaked += amount;
        self.save_account(&account_id, &account);
        self.last_total_balance += amount;

        env::log_str(
            format!(
                "@{} deposited {}. New unstaked balance is {}",
                account_id, amount, account.unstaked
            )
            .as_str(),
        );

        PromiseOrValue::Value(U128::from(0)) // no refund
    }
}
