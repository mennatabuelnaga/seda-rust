use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseError, Balance, PromiseOrValue, Promise};

use crate::fungible_token::{GAS_FOR_FT_ON_TRANSFER, ft};
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

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        assert!(amount > 0, "Withdrawal amount should be positive");

        let account_id = env::predecessor_account_id();
        let account = self.get_account(&account_id);
        assert!(account.unstaked >= amount, "Not enough unstaked balance to withdraw");
        assert!(
            account.unstaked_available_epoch_height <= env::epoch_height(),
            "The unstaked balance is not yet available due to unstaking delay"
        );

        // transfer the tokens, then validate/update state in `withdraw_callback()`
        ft::ext(self.seda_token.clone())
            .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
            .with_attached_deposit(1)
            .ft_transfer(account_id.clone(), amount.into(), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
                    .withdraw_callback(account_id, amount.into()),
            );
    }

    #[private] // require caller to be this contract
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        if call_result.is_err() {
            env::log_str("withdraw failed");
            return;
        }

        // update account
        let mut account = self.get_account(&account_id);
        account.unstaked -= amount.0;
        self.save_account(&account_id, &account);

        env::log_str(
            format!(
                "@{} withdrawing {}. New unstaked balance is {}",
                account_id, amount.0, account.unstaked
            )
            .as_str(),
        );

        // update global balance
        self.last_total_balance -= amount.0;
    }
}
