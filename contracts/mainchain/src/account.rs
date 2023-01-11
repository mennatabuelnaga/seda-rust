use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    AccountId,
    Balance,
};

use crate::{MainchainContract, U256};
/// A type to distinguish between a balance and "stake" shares for better
/// readability.
pub type NumStakeShares = Balance;

/// Inner account data of a staking delegate.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Default)]
pub struct Account {
    /// The unstaked balance. It represents the amount the account has on this
    /// contract that can either be staked or withdrawn.
    pub unstaked:                        Balance,
    /// The amount of "stake" shares. Every stake share corresponds to the
    /// amount of staked balance. NOTE: The number of shares should always
    /// be less or equal than the amount of staked balance. This means the
    /// price of stake share should always be at least `1`. The price of
    /// stake share can be computed as `total_staked_balance` /
    /// `total_stake_shares`.
    pub stake_shares:                    NumStakeShares,
    /// The minimum epoch height when the withdrawn is allowed.
    /// This changes after unstaking action, because the amount is still locked
    /// for 3 epochs.
    pub unstaked_available_epoch_height: u64,
}

/// Contract internal methods
impl MainchainContract {
    pub fn internal_get_account(&self, account_id: &AccountId) -> Account {
        self.accounts.get(account_id).unwrap_or_default()
    }

    pub fn internal_save_account(&mut self, account_id: &AccountId, account: &Account) {
        if account.unstaked > 0 || account.stake_shares > 0 {
            self.accounts.insert(account_id, account);
        } else {
            self.accounts.remove(account_id);
        }
    }

    pub(crate) fn internal_deposit(&mut self, amount: u128, account_id: AccountId) -> PromiseOrValue<U128> {
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

        PromiseOrValue::Value(U128::from(0)) // no refund
    }

    pub(crate) fn internal_stake(&mut self, amount: Balance) {
        assert!(amount > 0, "Staking amount should be positive");
        let account_id = env::signer_account_id();
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

    pub(crate) fn internal_unstake(&mut self, amount: Balance) {
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

    pub fn internal_withdraw(&mut self, amount: Balance) {
        let amount: Balance = amount.into();
        assert!(amount > 0, "Withdrawal amount should be positive");

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
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

    // /// Returns the number of "stake" shares rounded down corresponding to the
    // given staked balance /// amount.
    // ///
    // /// price = total_staked / total_shares
    // /// Price is fixed
    // /// (total_staked + amount) / (total_shares + num_shares) = total_staked /
    // total_shares /// (total_staked + amount) * total_shares = total_staked *
    // (total_shares + num_shares) /// amount * total_shares = total_staked *
    // num_shares /// num_shares = amount * total_shares / total_staked
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

    // /// Returns the staked amount rounded down corresponding to the given number
    // of "stake" shares.
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
}
