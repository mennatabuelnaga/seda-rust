use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
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
    pub fn get_account(&self, account_id: &AccountId) -> Account {
        self.accounts.get(account_id).unwrap_or_default()
    }

    pub fn save_account(&mut self, account_id: &AccountId, account: &Account) {
        if account.unstaked > 0 || account.stake_shares > 0 {
            self.accounts.insert(account_id, account);
        } else {
            self.accounts.remove(account_id);
        }
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
