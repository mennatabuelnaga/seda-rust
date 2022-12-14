use near_sdk::{near_bindgen, PromiseError, env, json_types::U128, AccountId};
use crate::{StakingContract, StakingContractExt};

/// Contract public methods
#[near_bindgen]
impl StakingContract {
    #[private]
    pub fn withdraw_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>, account_id: AccountId, need_to_restake: bool, amount: U128) {
        if call_result.is_err() {
            env::log_str("withdraw failed");
            return;
        }

        let mut account = self.internal_get_account(&account_id);
        account.unstaked -= amount.0;
        self.internal_save_account(&account_id, &account);

        env::log_str(
            format!(
                "@{} withdrawing {}. New unstaked balance is {}",
                account_id, amount.0, account.unstaked
            )
            .as_str(),
        );

        self.last_total_balance -= amount.0;

        if need_to_restake {
            self.internal_restake();
        }
    }
}
