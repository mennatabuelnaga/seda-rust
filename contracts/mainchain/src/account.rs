use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
};

use crate::{staking::NumStakeShares, MainchainContract};

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

/// Represents an account structure readable by humans.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableAccount {
    pub account_id:       AccountId,
    /// The unstaked balance that can be withdrawn or staked.
    pub unstaked_balance: U128,
    /// The amount balance staked at the current "stake" share price.
    pub staked_balance:   U128,
    /// Whether the unstaked balance is available for withdrawal now.
    pub can_withdraw:     bool,
}

/// Contract account private methods
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
}
