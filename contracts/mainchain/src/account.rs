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

    /*************** */
    /* View methods */
    /*************** */

    /// Returns the unstaked balance of the given account.
    pub fn get_account_unstaked_balance(&self, account_id: AccountId) -> U128 {
        self.get_human_readable_account(account_id).unstaked_balance
    }

    /// Returns the staked balance of the given account.
    /// NOTE: This is computed from the amount of "stake" shares the given
    /// account has and the current amount of total staked balance and total
    /// stake shares on the account.
    pub fn get_account_staked_balance(&self, account_id: AccountId) -> U128 {
        self.get_human_readable_account(account_id).staked_balance
    }

    /// Returns the total balance of the given account (including staked and
    /// unstaked balances).
    pub fn get_account_total_balance(&self, account_id: AccountId) -> U128 {
        let account = self.get_human_readable_account(account_id);
        (account.unstaked_balance.0 + account.staked_balance.0).into()
    }

    /// Returns `true` if the given account can withdraw tokens in the current
    /// epoch.
    pub fn is_account_unstaked_balance_available(&self, account_id: AccountId) -> bool {
        self.get_human_readable_account(account_id).can_withdraw
    }

    /// Returns the total staking balance.
    pub fn get_total_staked_balance(&self) -> U128 {
        self.total_staked_balance.into()
    }

    /// Returns human readable representation of the account for the given
    /// account ID.
    pub fn get_human_readable_account(&self, account_id: AccountId) -> HumanReadableAccount {
        let account = self.internal_get_account(&account_id);
        HumanReadableAccount {
            account_id,
            unstaked_balance: account.unstaked.into(),
            staked_balance: self
                .staked_amount_from_num_shares_rounded_down(account.stake_shares)
                .into(),
            can_withdraw: account.unstaked_available_epoch_height <= env::epoch_height(),
        }
    }

    /// Returns the number of accounts that have positive balance on this
    /// staking pool.
    pub fn get_number_of_accounts(&self) -> u64 {
        self.accounts.len()
    }

    /// Returns the list of accounts
    pub fn get_human_readable_accounts(&self, from_index: u64, limit: u64) -> Vec<HumanReadableAccount> {
        let keys = self.accounts.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.get_human_readable_account(keys.get(index).unwrap()))
            .collect()
    }
}
