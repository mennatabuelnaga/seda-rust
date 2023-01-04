use fungible_token::{ft, GAS_FOR_FT_ON_TRANSFER};
use near_sdk::serde_json::json;

use crate::{callbacks::PingAction, *};

/// Contract internal methods
impl StakingContract {
    pub(crate) fn internal_deposit(&mut self, amount: u128, account_id: AccountId) -> u128 {
        env::log_str(format!("account_id is {}", account_id).as_str());
        let mut account = self.internal_get_account(&account_id);
        account.unstaked += amount;
        self.internal_save_account(&account_id, &account);
        self.last_total_balance += amount;

        env::log_str(
            format!(
                "@{} deposited {}. New unstaked balance is {}",
                account_id, amount, account.unstaked
            )
            .as_str(),
        );
        amount
    }

    /// Perform checks for valid withdraw action, calls `ft_transfer` on token,
    /// then uses `withdraw_callback` to update state
    pub(crate) fn internal_withdraw(&mut self, amount: Balance, account_id: AccountId) -> Promise {
        assert!(amount > 0, "Withdrawal amount should be positive");

        let account = self.internal_get_account(&account_id);
        assert!(account.unstaked >= amount, "Not enough unstaked balance to withdraw");
        assert!(
            account.unstaked_available_epoch_height <= env::epoch_height(),
            "The unstaked balance is not yet available due to unstaking delay"
        );

        ft::ext(self.seda_token.clone())
            .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
            .with_attached_deposit(1)
            .ft_transfer(account_id.clone(), amount.into(), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
                    .withdraw_callback(account_id, amount.into()),
            )
    }

    pub(crate) fn internal_stake(&mut self, amount: Balance, account_id: AccountId) {
        assert!(amount > 0, "Staking amount should be positive");

        let mut account = self.internal_get_account(&account_id);

        // Calculate the number of "stake" shares that the account will receive for
        // staking the given amount.
        let num_shares = self.num_shares_from_staked_amount_rounded_down(amount);
        assert!(
            num_shares > 0,
            "The calculated number of \"stake\" shares received for staking should be positive"
        );
        // The amount of tokens the account will be charged from the unstaked balance.
        // Rounded down to avoid overcharging the account to guarantee that the account
        // can always unstake at least the same amount as staked.
        let charge_amount = self.staked_amount_from_num_shares_rounded_down(num_shares);
        assert!(
            charge_amount > 0,
            "Invariant violation. Calculated staked amount must be positive, because \"stake\" share price should be at least 1"
        );

        assert!(
            account.unstaked >= charge_amount,
            "Not enough unstaked balance to stake"
        );
        account.unstaked -= charge_amount;
        account.stake_shares += num_shares;
        self.internal_save_account(&account_id, &account);

        // The staked amount that will be added to the total to guarantee the "stake"
        // share price never decreases. The difference between `stake_amount`
        // and `charge_amount` is paid from the allocated
        // STAKE_SHARE_PRICE_GUARANTEE_FUND.
        let stake_amount = self.staked_amount_from_num_shares_rounded_up(num_shares);

        self.total_staked_balance += stake_amount;
        self.total_stake_shares += num_shares;

        env::log_str(
            format!(
                "@{} staking {}. Received {} new staking shares. Total {} unstaked balance and {} staking shares",
                account_id, charge_amount, num_shares, account.unstaked, account.stake_shares
            )
            .as_str(),
        );
        env::log_str(
            format!(
                "Contract total staked balance is {}. Total number of shares {}",
                self.total_staked_balance, self.total_stake_shares
            )
            .as_str(),
        );
    }

    pub(crate) fn inner_unstake(&mut self, amount: u128) {
        assert!(amount > 0, "Unstaking amount should be positive");

        let account_id = env::predecessor_account_id();
        let mut account = self.internal_get_account(&account_id);

        assert!(
            self.total_staked_balance > 0,
            "The contract doesn't have staked balance"
        );
        // Calculate the number of shares required to unstake the given amount.
        // NOTE: The number of shares the account will pay is rounded up.
        let num_shares = self.num_shares_from_staked_amount_rounded_up(amount);
        assert!(
            num_shares > 0,
            "Invariant violation. The calculated number of \"stake\" shares for unstaking should be positive"
        );
        assert!(
            account.stake_shares >= num_shares,
            "Not enough staked balance to unstake"
        );

        // Calculating the amount of tokens the account will receive by unstaking the
        // corresponding number of "stake" shares, rounding up.
        let receive_amount = self.staked_amount_from_num_shares_rounded_up(num_shares);
        assert!(
            receive_amount > 0,
            "Invariant violation. Calculated staked amount must be positive, because \"stake\" share price should be at least 1"
        );

        account.stake_shares -= num_shares;
        account.unstaked += receive_amount;
        account.unstaked_available_epoch_height = env::epoch_height() + NUM_EPOCHS_TO_UNLOCK;
        self.internal_save_account(&account_id, &account);

        // The amount tokens that will be unstaked from the total to guarantee the
        // "stake" share price never decreases. The difference between
        // `receive_amount` and `unstake_amount` is paid from the allocated
        // STAKE_SHARE_PRICE_GUARANTEE_FUND.
        let unstake_amount = self.staked_amount_from_num_shares_rounded_down(num_shares);

        self.total_staked_balance -= unstake_amount;
        self.total_stake_shares -= num_shares;

        env::log_str(
            format!(
                "@{} unstaking {}. Spent {} staking shares. Total {} unstaked balance and {} staking shares",
                account_id, receive_amount, num_shares, account.unstaked, account.stake_shares
            )
            .as_str(),
        );
        env::log_str(
            format!(
                "Contract total staked balance is {}. Total number of shares {}",
                self.total_staked_balance, self.total_stake_shares
            )
            .as_str(),
        );
    }

    /// Asserts that the method was called by the owner.
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    /// Distributes rewards after the new epoch. It's automatically called
    /// before every action. Returns true if the current epoch height is
    /// different from the last epoch height.
    pub(crate) fn internal_ping(&mut self, action: PingAction) -> Promise {
        let get_epoch_promise = Promise::new(self.mainchain_contract.clone()).function_call(
            "get_epoch".to_owned(),
            vec![],
            0,
            GAS_FOR_FT_ON_TRANSFER, // TODO
        );

        let staking_pool_balance_promise = Promise::new(self.seda_token.clone()).function_call(
            "ft_balance_of".to_owned(),
            json!({ "account_id": env::current_account_id() })
                .to_string()
                .into_bytes(),
            0,
            GAS_FOR_FT_ON_TRANSFER, // TODO
        );

        get_epoch_promise.and(staking_pool_balance_promise).then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                .ping_callback(action),
        )
    }

    /// Returns the number of "stake" shares rounded down corresponding to the
    /// given staked balance amount.
    ///
    /// price = total_staked / total_shares
    /// Price is fixed
    /// (total_staked + amount) / (total_shares + num_shares) = total_staked /
    /// total_shares (total_staked + amount) * total_shares = total_staked *
    /// (total_shares + num_shares) amount * total_shares = total_staked *
    /// num_shares num_shares = amount * total_shares / total_staked
    pub(crate) fn num_shares_from_staked_amount_rounded_down(&self, amount: Balance) -> NumStakeShares {
        assert!(self.total_staked_balance > 0, "The total staked balance can't be 0");
        (U256::from(self.total_stake_shares) * U256::from(amount) / U256::from(self.total_staked_balance)).as_u128()
    }

    /// Returns the number of "stake" shares rounded up corresponding to the
    /// given staked balance amount.
    ///
    /// Rounding up division of `a / b` is done using `(a + b - 1) / b`.
    pub(crate) fn num_shares_from_staked_amount_rounded_up(&self, amount: Balance) -> NumStakeShares {
        assert!(self.total_staked_balance > 0, "The total staked balance can't be 0");
        ((U256::from(self.total_stake_shares) * U256::from(amount) + U256::from(self.total_staked_balance - 1))
            / U256::from(self.total_staked_balance))
        .as_u128()
    }

    /// Returns the staked amount rounded down corresponding to the given number
    /// of "stake" shares.
    pub(crate) fn staked_amount_from_num_shares_rounded_down(&self, num_shares: NumStakeShares) -> Balance {
        assert!(
            self.total_stake_shares > 0,
            "The total number of stake shares can't be 0"
        );
        (U256::from(self.total_staked_balance) * U256::from(num_shares) / U256::from(self.total_stake_shares)).as_u128()
    }

    /// Returns the staked amount rounded up corresponding to the given number
    /// of "stake" shares.
    ///
    /// Rounding up division of `a / b` is done using `(a + b - 1) / b`.
    pub(crate) fn staked_amount_from_num_shares_rounded_up(&self, num_shares: NumStakeShares) -> Balance {
        assert!(
            self.total_stake_shares > 0,
            "The total number of stake shares can't be 0"
        );
        ((U256::from(self.total_staked_balance) * U256::from(num_shares) + U256::from(self.total_stake_shares - 1))
            / U256::from(self.total_stake_shares))
        .as_u128()
    }

    /// Inner method to get the given account or a new default value account.
    pub(crate) fn internal_get_account(&self, account_id: &AccountId) -> Account {
        self.accounts.get(account_id).unwrap_or_default()
    }

    /// Inner method to save the given account for a given account ID.
    /// If the account balances are 0, the account is deleted instead to release
    /// storage.
    pub(crate) fn internal_save_account(&mut self, account_id: &AccountId, account: &Account) {
        if account.unstaked > 0 || account.stake_shares > 0 {
            self.accounts.insert(account_id, account);
        } else {
            self.accounts.remove(account_id);
        }
    }
}
